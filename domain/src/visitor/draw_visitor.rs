use std::collections::HashMap;
use std::ops::Sub;
use std::sync::{Arc, Mutex};

use egui::{Color32, Pos2, Stroke, TextureId};
use glam::{Vec3, Vec4, Vec4Swizzles};
use log::debug;

use crate::canvas::painter::Painter3D;
use crate::math::Transform;
use crate::object::camera::Camera;
use crate::object::objects::cloud::{beer, hg, phase};
use crate::object::objects::{BoundingBox, Cloud, Grid, Sun, Terrain};
use crate::scene::scene_composite::SceneObjects;
use crate::visitor::{Visitable, Visitor};

pub struct DrawVisitor<'a> {
    canvas: &'a Painter3D,
    camera: &'a Camera,
    stroke: Stroke,
    mvp: Transform,
}

impl<'a> DrawVisitor<'a> {
    pub fn new(camera: &'a Camera, canvas: &'a Painter3D) -> Self {
        let resp_rect = canvas.resp_rect().sub((-8.0).into());
        let proj = camera.projection(resp_rect.width(), resp_rect.height());
        let camera_tf = proj * camera.view();

        let res = Self {
            canvas,
            camera,
            stroke: Stroke::new(1.0, Color32::GRAY),
            mvp: Transform::new(camera_tf, resp_rect),
        };
        res.visit_sky();
        res
    }

    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }
}

impl<'a> Visitor for DrawVisitor<'a> {
    fn visit_composite(&mut self, scene_objects: &SceneObjects) {
        let mut objs = scene_objects.values().map(|x| x).collect::<Vec<_>>();
        objs.sort_by(|x, y| {
            (y.pos() - self.camera.pos())
                .length()
                .partial_cmp(&(x.pos() - self.camera.pos()).length())
                .unwrap()
        });
        for i in objs {
            i.accept(self);
        }
    }

    fn visit_camera(&mut self, _camera: &Camera) {
        debug!("Visit camera {:?}", self.camera);
    }

    fn visit_cloud(&mut self, cloud: &Cloud) {
        self.canvas
            .ctx()
            .data_mut(|x| x.insert_temp("cloud".into(), cloud.clone()));
        use rayon::prelude::*;

        let bb = cloud.bounding_box();
        let corners = bb.corners();

        let (width, height) = (1056.0, 900.0);

        let (min_tuple, max_tuple) = corners
            .iter()
            .enumerate()
            .map(|(i, &corner)| {
                self.canvas.transform(corner, self.mvp).unwrap_or_else(|| {
                    if i < 4 {
                        Pos2::new(width, height)
                    } else {
                        Pos2::ZERO
                    }
                })
            })
            .fold(
                (Pos2::new(width, height), Pos2::new(0.0, 0.0)),
                |(mut min, mut max), p| {
                    min.x = min.x.min(p.x).clamp(0.0, width);
                    min.y = min.y.min(p.y).clamp(0.0, height);
                    max.x = max.x.max(p.x).clamp(0.0, width);
                    max.y = max.y.max(p.y).clamp(0.0, height);
                    (min, max)
                },
            );

        let wh = max_tuple - min_tuple;
        let (w, h) = (wh.x as usize, wh.y as usize);

        let mut img = egui::ColorImage::new([w, h], Color32::TRANSPARENT);
        let background_color: Vec4 = Color32::TRANSPARENT
            .to_array()
            .map(|x| x as f32 / 255.0)
            .into();
        let col_a: Vec4 = cloud.col_a.to_array().map(|x| x as f32 / 255.0).into();
        let col_b: Vec4 = cloud.col_b.to_array().map(|x| x as f32 / 255.0).into();
        let light_color: Vec4 = cloud
            .light_color
            .to_array()
            .map(|x| x as f32 / 255.0)
            .into();

        let sun = self
            .canvas
            .ctx()
            .data_mut(|x| x.get_persisted::<Sun>("sun".into()));
        let sun = sun.unwrap_or_default();
        let sun_pos = sun.get_pos();
        let ray_origin = self.camera.pos();
        img.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let i = idx / w + min_tuple.y as usize;
                let j = idx % w + min_tuple.x as usize;

                let ray_dir = (self.camera.egui_to_world(i, j, 1056, 900) - ray_origin).normalize();

                let ray_box_info = bb.dst(ray_origin, ray_dir);
                let dst_to_box = ray_box_info.x;
                let dst_inside_box = ray_box_info.y;

                if dst_inside_box <= 0.0 {
                    *pixel = Color32::TRANSPARENT
                } else {
                    let mut dst_travelled = 0.0;
                    let dst_limit = dst_inside_box;
                    let step_size = dst_inside_box / cloud.num_steps as f32;
                    let mut transmittance = 1.0;
                    let mut light_energy = 0.0;

                    let entry_point = ray_origin + dst_to_box * ray_dir;
                    let cos_angle = ray_dir.dot(sun_pos);
                    let phase = phase(cos_angle.abs(), cloud.phase_params);

                    while dst_travelled < dst_limit {
                        let ray_pos = entry_point + ray_dir * dst_travelled;
                        let density = cloud.sample_density(ray_pos);
                        if density > 0.1 {
                            let light_transmittance = cloud.light_march(ray_pos, sun_pos);
                            light_energy +=
                                density * step_size * transmittance * light_transmittance * phase;
                            transmittance *=
                                beer(density * step_size * cloud.light_absorption_through_cloud);
                            if transmittance < 0.01 {
                                break;
                            }
                        }
                        dst_travelled += step_size;
                    }

                    *pixel = if 1.0 - transmittance == 0.0 {
                        Color32::TRANSPARENT
                    } else {
                        let background_color = Vec3::ZERO;

                        let focused_eye_cos = cos_angle
                            .clamp(-std::f32::consts::PI, std::f32::consts::PI)
                            .powf(cloud.params.x);
                        let sun = hg(focused_eye_cos, 0.4).clamp(-1.0, 1.0) * transmittance;

                        let cloud_col = light_energy * light_color.xyz();
                        let col = background_color * transmittance + cloud_col;
                        let col = col.clamp(Vec3::ZERO, Vec3::ONE) * (1.0 - sun)
                            + light_color.xyz() * sun;
                        let (r, g, b) = col.into();
                        Color32::from_rgba_unmultiplied(
                            (r * 255.0) as u8,
                            (g * 255.0) as u8,
                            (b * 255.0) as u8,
                            (255.0 * (1.0 - transmittance)) as u8,
                        )
                    };
                }
            });

        let handle = self
            .canvas
            .ctx()
            .load_texture("cloud", img, Default::default());
        let textureid = TextureId::from(&handle);
        self.canvas.image(
            textureid,
            egui::Rect::from_two_pos(min_tuple, max_tuple),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
        // self.visit_bounding_box(bb);
    }

    fn visit_grid(&mut self, grid: &Grid) {
        let k = grid.k;
        let scale = grid.scale;
        let f = k as f32;
        for i in -k..=k {
            self.canvas.line(
                Vec3::new(-1., 0., i as f32 / f) * scale,
                Vec3::new(1., 0., i as f32 / f) * scale,
                Stroke::new(1.0, Color32::BLACK),
                self.mvp,
            );

            self.canvas.line(
                Vec3::new(i as f32 / f, 0., -1.) * scale,
                Vec3::new(i as f32 / f, 0., 1.) * scale,
                Stroke::new(1.0, Color32::BLACK),
                self.mvp,
            );
        }

        self.canvas.line(
            Vec3::new(0., 0., 0.) * scale,
            Vec3::new(0., 1.4, 0.) * scale,
            Stroke::new(2.0, Color32::DARK_GREEN),
            self.mvp,
        );

        self.canvas.line(
            Vec3::new(0., 0., 0.) * scale,
            Vec3::new(0., 0., 1.4) * scale,
            Stroke::new(2.0, Color32::BLUE),
            self.mvp,
        );

        self.canvas.line(
            Vec3::new(0., 0., 0.) * scale,
            Vec3::new(1.4, 0., 0.) * scale,
            Stroke::new(2.0, Color32::RED),
            self.mvp,
        );

        self.canvas.text(
            Vec3::new(1.5, 0., 0.),
            egui::Align2::CENTER_BOTTOM,
            "x",
            egui::FontId::monospace(14.0),
            Color32::RED,
            self.mvp,
        );

        self.canvas.text(
            Vec3::new(0., 1.5, 0.),
            egui::Align2::CENTER_BOTTOM,
            "y",
            egui::FontId::monospace(14.0),
            Color32::DARK_GREEN,
            self.mvp,
        );

        self.canvas.text(
            Vec3::new(0., 0., 1.5),
            egui::Align2::CENTER_BOTTOM,
            "z",
            egui::FontId::monospace(14.0),
            Color32::BLUE,
            self.mvp,
        );
    }

    fn visit_bounding_box(&mut self, bb: &BoundingBox) {
        let (x1, y1, z1) = bb.min.into();
        let (x2, y2, z2) = bb.max.into();
        let corners = [
            (x1, y1, z1),
            (x2, y1, z1),
            (x1, y2, z1),
            (x1, y1, z2),
            (x2, y2, z1),
            (x2, y1, z2),
            (x1, y2, z2),
            (x2, y2, z2),
        ];
        let edges = [
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 4),
            (2, 4),
            (1, 5),
            (3, 5),
            (2, 6),
            (3, 6),
            (4, 7),
            (6, 7),
            (5, 7),
        ];
        for &(i1, i2) in &edges {
            let (x1, y1, z1) = corners[i1];
            let (x2, y2, z2) = corners[i2];
            self.canvas.dashed_line(
                Vec3::new(x1, y1, z1),
                Vec3::new(x2, y2, z2),
                1.0,
                0.5,
                Stroke::new(1.0, Color32::DARK_RED),
                self.mvp,
            );
        }
    }

    fn visit_sun(&mut self, sun: &Sun) {
        let sun_pos = sun.get_pos();
        self.canvas
            .ctx()
            .data_mut(|x| x.insert_persisted("sun".into(), sun.clone()));
        self.canvas.circle_filled(
            sun_pos,
            sun.get_pos() + Vec3::new(0.1, 0.0, 0.0),
            Color32::LIGHT_YELLOW,
            self.mvp,
        );
    }

    fn visit_terrain(&mut self, terrain: &Terrain) {
        
        use rayon::prelude::*;

        let sun = self
            .canvas
            .ctx()
            .data_mut(|x| x.get_persisted::<Sun>("sun".into()));

        if sun.is_none() {
            return;
        }
        let sun_pos = sun.unwrap().get_pos();
        let cloud = self
            .canvas
            .ctx()
            .data_mut(|x| x.get_temp::<Cloud>("cloud".into()))
            .unwrap_or_default();

        let (width, height) = (1056.0, 900.0);
        let (min_tuple, max_tuple) = (Pos2::ZERO, Pos2::new(width, height));
        let wh = max_tuple - min_tuple;
        let (w, h) = (wh.x as usize, wh.y as usize);
        let bb = terrain.bounding_box;

        let img = Arc::new(Mutex::new(egui::ColorImage::new(
            [w, h],
            Color32::TRANSPARENT,
        )));
        let z_buffer = Arc::new(Mutex::new(HashMap::new()));

        terrain.triangles.par_iter().for_each(|(v, (n0, n1, n2))| {
            let img = img.clone();
            let z_buffer = z_buffer.clone();
            let get_shadow_factor = |probe: Vec3| -> f32 {
                let sun_dir = (sun_pos - probe).normalize();
                let cloud_bb = cloud.bounding_box().dst(probe, sun_dir);
                let (dir_to_box, dst_inside_box) = cloud_bb.into();
                if dst_inside_box != 0.0 {
                    let mut p = probe;
                    let num_steps = terrain.num_shadows_steps;
                    let step_size = dst_inside_box / num_steps as f32;
                    p += dir_to_box * sun_dir;

                    let mut total_density = 0.0;

                    for _ in 0..num_steps {
                        let density = cloud.sample_density(p);
                        total_density += density.max(0.0) * step_size;
                        p += sun_dir * step_size;
                    }
                    beer(total_density / terrain.density_scale)
                        .clamp(terrain.shadow_threshold, 1.0)
                } else {
                    1.0
                }
            };

            let (v0, v1, v2) = v.to_tuple();
            let (p0, p1, p2) = (v0, v1, v2);

            let v0 = self.canvas.transform(v0, self.mvp);
            let v1 = self.canvas.transform(v1, self.mvp);
            let v2 = self.canvas.transform(v2, self.mvp);
            if let (Some(v0), Some(v1), Some(v2)) = (v0, v1, v2) {
                let min_x = v0.x.min(v1.x).min(v2.x) as usize;
                let max_x = v0.x.max(v1.x).max(v2.x) as usize;
                let min_y = v0.y.min(v1.y).min(v2.y) as usize;
                let max_y = v0.y.max(v1.y).max(v2.y) as usize;

                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        if inside_triangle(Pos2::new(x as f32, y as f32), v0, v1, v2)
                            && x < width as usize
                            && y < height as usize
                        {
                            let a1 = Vec3::new(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
                            let a2 = Vec3::new(p2.x - p0.x, p2.y - p0.y, p2.z - p0.z);
                            let normal = a1.cross(a2);
                            let d = -(normal.x * p0.x + normal.y * p0.y + normal.z * p0.z);
                            let (a, b, c, d) = (normal.x, normal.y, normal.z, d);

                            let new_u = p1.x + (p0.x - p1.x) * ((x as f32 - v1.x) / (v0.x - v1.x));
                            let new_v = p1.y + (p0.y - p1.y) * ((y as f32 - v1.y) / (v0.y - v1.y));
                            let z_pixel = if c != 0.0 {
                                -(a * new_u + b * new_v + d) / c
                            } else {
                                0.0
                            };

                            let probe = Vec3::new(new_u, new_v, z_pixel);
                            let depth = self.camera.pos().distance(probe);

                            let x1 = get_shadow_factor(p0);
                            let x2 = get_shadow_factor(p1);
                            let x3 = get_shadow_factor(p2);
                            
                            let alpha1 = ((sun_pos - p0).normalize()).dot(n0.normalize());
                            let alpha2 = ((sun_pos - p1).normalize()).dot(n1.normalize());
                            let alpha3 = ((sun_pos - p2).normalize()).dot(n2.normalize());

                            let beta =
                                interpolate(Pos2::new(x as f32, y as f32), v0, v1, v2, x1 * alpha1, x2 * alpha2, x3 * alpha3)
                                ;

                            let dif = 0.55  * beta;
                            let bottom = color32_to_vec4(terrain.bottom_color).xyz();
                            let top = color32_to_vec4(terrain.top_color).xyz();
                            let p0_col = bottom.lerp(top, (p0.y - bb.min.y).abs() / bb.size().y);
                            let p1_col = bottom.lerp(top, (p1.y - bb.min.y).abs() / bb.size().y);
                            let p2_col = bottom.lerp(top, (p2.y - bb.min.y).abs() / bb.size().y);

                            let col = interpolate(
                                Pos2::new(x as f32, y as f32),
                                v0,
                                v1,
                                v2,
                                p0_col,
                                p1_col,
                                p2_col,
                            );
                            // let col = Vec3::new(0.71, 0.94, 0.73) * dif;
                            let col = col * dif;
                            // let col = Vec3::new(0.71, 0.94, 0.73) * dif;
                            let (r, g, b) = (col.x * 255.0, col.y * 255.0, col.z * 255.0);
                            let color = Color32::from_rgb(r as u8, g as u8, b as u8);
                            let mut z_buffer = z_buffer.lock().unwrap();
                            if let Some(existing_depth) = z_buffer.get(&(x, y)) {
                                if depth < *existing_depth {
                                    img.lock().unwrap()[(x, y)] = color;
                                    z_buffer.insert((x, y), depth);
                                }
                            } else {
                                img.lock().unwrap()[(x, y)] = color;
                                z_buffer.insert((x, y), depth);
                            }
                            drop(z_buffer);
                        }
                    }
                }
            }
        });

        let img = Arc::try_unwrap(img)
            .expect("one strong reference")
            .into_inner()
            .expect("No one holding the mutex");
        let handle = self
            .canvas
            .ctx()
            .load_texture("terrain", img, egui::TextureOptions::NEAREST);
        let textureid = TextureId::from(&handle);
        self.canvas.image(
            textureid,
            egui::Rect::from_two_pos(min_tuple, max_tuple),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );

        // self.visit_bounding_box(&terrain.bounding_box);
    }
}

impl<'a> DrawVisitor<'a> {
    fn visit_sky(&self) {
        use rayon::prelude::*;
        let (width, height) = (1066.0, 950.0);
        let (min_tuple, max_tuple) = (Pos2::ZERO, Pos2::new(width, height));
        let wh = max_tuple - min_tuple;
        let (w, h) = (wh.x as usize, wh.y as usize);

        let sun = self
            .canvas
            .ctx()
            .data_mut(|x| x.get_persisted::<Sun>("sun".into()));
        if sun.is_none() {
            return;
        }
        let sun = sun.unwrap();

        let mut img = egui::ColorImage::new([w, h], Color32::BLACK);
        img.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let i = idx / w + min_tuple.y as usize;
                let j = idx % w + min_tuple.x as usize;
                let uv = glam::Vec2::new(j as f32, 900.0 - i as f32) / 900.0;

                let atmosphere = (1.0_f32 - uv.y).sqrt();
                let sky_col = Vec3::new(0.2, 0.4, 0.8);

                let scatter = sun.get_pos().y / sun.d;
                let scatter = scatter.powf(1.0 / 10.0);
                let scatter = 1.0 - scatter.clamp(0.2, 0.9);

                let col = Vec3::splat(1.0).lerp(Vec3::new(0.8, 0.3, 0.0) * 1.1, scatter);
                let col = sky_col.lerp(col, atmosphere / 1.1);
                let col = col;
                let (r, g, b) = col.into();
                *pixel = Color32::from_rgba_unmultiplied(
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                    255,
                );
            });

        let handle = self
            .canvas
            .ctx()
            .load_texture("terrain", img, Default::default());
        let textureid = TextureId::from(&handle);
        self.canvas.image(
            textureid,
            egui::Rect::from_two_pos(min_tuple, max_tuple),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }
}

fn interpolate<T>(pos: Pos2, v0: Pos2, v1: Pos2, v2: Pos2, n0: T, n1: T, n2: T) -> T
where
    T: std::ops::Mul<f32, Output = T> + std::ops::Add<Output = T>,
{
    let area_total = (v1.x - v0.x) * (v2.y - v0.y) - (v2.x - v0.x) * (v1.y - v0.y);
    let alpha = ((v1.x - pos.x) * (v2.y - pos.y) - (v2.x - pos.x) * (v1.y - pos.y)) / area_total;
    let beta = ((v2.x - pos.x) * (v0.y - pos.y) - (v0.x - pos.x) * (v2.y - pos.y)) / area_total;
    let gamma = 1.0 - alpha - beta;

    n0 * alpha + n1 * beta + n2 * gamma
}

#[inline]
fn sign(p1: Pos2, p2: Pos2, p3: Pos2) -> f32 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}

fn inside_triangle(p: Pos2, v1: Pos2, v2: Pos2, v3: Pos2) -> bool {
    let d1 = sign(p, v1, v2);
    let d2 = sign(p, v2, v3);
    let d3 = sign(p, v3, v1);

    let has_neg = (d1 < 0.) || (d2 < 0.) || (d3 < 0.);
    let has_pos = (d1 > 0.) || (d2 > 0.) || (d3 > 0.);

    !(has_neg && has_pos)
}

#[inline]
fn color32_to_vec4(color32: Color32) -> Vec4 {
    color32.to_array().map(|x| x as f32 / 255.0).into()
}

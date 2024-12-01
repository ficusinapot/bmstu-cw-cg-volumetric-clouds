use std::cmp::max;
use std::collections::HashMap;
use std::ops::Neg;
use std::sync::{Arc, Mutex};

use egui::{Color32, ColorImage, Pos2, Stroke, TextureId};
use egui::ahash::AHashMap;
use glam::{Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use log::debug;
use rand::{random, Rng, thread_rng};

use crate::canvas::painter::Painter3D;
use crate::math::Transform;
use crate::object::camera::Camera;
use crate::object::objects::{BoundingBox, Cloud, Grid, Sun, Terrain};
use crate::object::objects::cloud::{beer, hg, phase};
use crate::visitor::Visitor;

pub struct DrawVisitor<'a> {
    color_image: &'a mut ColorImage,
    canvas: &'a Painter3D,
    camera: &'a Camera,
    stroke: Stroke,
    background_color: Color32,
    mvp: Transform,
    hash_map: HashMap<(usize, usize), usize>,
}

impl<'a> DrawVisitor<'a> {
    pub fn new(color_image: &'a mut ColorImage, camera: &'a Camera, canvas: &'a Painter3D) -> Self {
        let resp_rect = canvas.resp_rect();
        let proj = camera.projection(resp_rect.width(), resp_rect.height());
        let camera_tf = proj * camera.view();

        Self {
            canvas,
            camera,
            color_image,
            stroke: Stroke::new(1.0, Color32::GRAY),
            background_color: Color32::WHITE,
            mvp: Transform::new(camera_tf, resp_rect),
            hash_map: HashMap::new(),
        }
    }

    pub fn with_color(mut self, color32: Color32) -> Self {
        self.background_color = color32;
        self
    }

    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }
}

impl<'a> Visitor for DrawVisitor<'a> {
    fn visit_camera(&mut self, _camera: &Camera) {
        debug!("Visit camera {:?}", self.camera);
    }

    fn visit_cloud(&mut self, cloud: &Cloud) {
        self.canvas
            .ctx()
            .data_mut(|x| x.insert_persisted("cloud".into(), cloud.clone()));
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
        let sun_pos = sun.map(|x| x.get_pos()).unwrap_or_default();
        let ray_origin = self.camera.get_position();

        img.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let i = idx / w + min_tuple.y as usize;
                let j = idx % w + min_tuple.x as usize;

                let ray_dir =
                    (self.camera.pixel_to_world(i, j, 1056, 900) - ray_origin).normalize();

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
                    let mut light_energy = Vec3::ZERO;

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

                    *pixel = if transmittance >= 0.03 {
                        Color32::TRANSPARENT
                    } else {
                        let sky_col_base =
                            col_a.lerp(col_b, ray_dir.y.clamp(0.0, 1.0).abs().sqrt());
                        let dst_fog = 0.5;
                        let sky = dst_fog * sky_col_base;
                        let background_color = background_color * (1.0 - dst_fog) + sky;

                        let focused_eye_cos = cos_angle
                            .clamp(-std::f32::consts::PI, std::f32::consts::PI)
                            .powf(cloud.params.x);
                        let sun = hg(focused_eye_cos, 0.4).clamp(-1.0, 1.0) * transmittance;

                        let cloud_col = light_energy * light_color.xyz();
                        let col = background_color.xyz() * transmittance + cloud_col;
                        let col = col.clamp(Vec3::ZERO, Vec3::ONE) * (1.0 - sun)
                            + light_color.xyz() * sun;
                        let (r, g, b) = col.into();
                        Color32::from_rgba_unmultiplied(
                            (r * 255.0) as u8,
                            (g * 255.0) as u8,
                            (b * 255.0) as u8,
                            255,
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
        // let cloud = self.canvas.ctx().data_mut(|x| x.get_persisted::<Cloud>("cloud".into())).unwrap();

        let (width, height) = (1056.0, 900.0);
        let (min_tuple, max_tuple) = (Pos2::ZERO, Pos2::new(width, height));
        let wh = max_tuple - min_tuple;
        let (w, h) = (wh.x as usize, wh.y as usize);

        let mut img = egui::ColorImage::new([w, h], Color32::TRANSPARENT);
        let mut z_buffer: HashMap<(usize, usize), f32> = HashMap::new();

        for (v, (n0, n1, n2)) in &terrain.triangles {
            let center = v.center();
            let light = (sun.unwrap().get_pos() - center).normalize();

            let (v0, v1, v2) = v.to_tuple();
            let (p1, p2, p3) = (v0, v1, v2);
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
                            let interpolated_normal = interpolate_normal(
                                Pos2::new(x as f32, y as f32),
                                v0,
                                v1,
                                v2,
                                n0.clone(),
                                n1.clone(),
                                n2.clone(),
                            );
                            let alpha = light.dot(interpolated_normal);
                            let dif = 0.55 * alpha;
                            let col = Vec3::new(0.71, 0.94, 0.73) * dif;
                            let (r, g, b) = (col.x * 255.0, col.y * 255.0, col.z * 255.0);
                            let color =
                                Color32::from_rgba_unmultiplied(r as u8, g as u8, b as u8, 255);

                            let v1 = Vec3::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
                            let v2 = Vec3::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);
                            let normal = v1.cross(v2);
                            let d = -(normal.x * p1.x + normal.y * p1.y + normal.z * p1.z);

                            let (a, b, c, d) = (normal.x, normal.y, normal.z, d);

                            // let z = self
                            //     .camera
                            //     .pixel_to_world(x, y, width as usize, height as usize)
                            //     .z;

                            let z = -(a * x as f32 + b * y as f32 + d) / c;

                            let pixel_index = (x, y);
                            if let Some(&existing_z) = z_buffer.get(&pixel_index) {
                                if z <= existing_z {
                                    z_buffer.insert(pixel_index, z);
                                    img[(x, y)] = color;
                                }
                            } else {
                                z_buffer.insert(pixel_index, z);
                                img[(x, y)] = color;
                            }
                            // println!("{:?}", z_buffer);
                        }
                    }
                }
            }
        }

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

fn interpolate_normal(
    pos: Pos2,
    v0: Pos2,
    v1: Pos2,
    v2: Pos2,
    n0: Vec3,
    n1: Vec3,
    n2: Vec3,
) -> Vec3 {
    let area_total = (v1.x - v0.x) * (v2.y - v0.y) - (v2.x - v0.x) * (v1.y - v0.y);
    let alpha = ((v1.x - pos.x) * (v2.y - pos.y) - (v2.x - pos.x) * (v1.y - pos.y)) / area_total;
    let beta = ((v2.x - pos.x) * (v0.y - pos.y) - (v0.x - pos.x) * (v2.y - pos.y)) / area_total;
    let gamma = 1.0 - alpha - beta;

    (n0 * alpha + n1 * beta + n2 * gamma).normalize()
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

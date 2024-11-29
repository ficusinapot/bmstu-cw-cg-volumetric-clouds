use std::sync::{Arc, Mutex};

use egui::{Color32, Pos2, Stroke, TextureId};
use glam::{Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use log::debug;
use rand::{random, thread_rng, Rng};

use crate::canvas::painter::Painter3D;
use crate::math::Transform;
use crate::object::camera::Camera;
use crate::object::objects::cloud::{beer, hg, phase};
use crate::object::objects::{BoundingBox, Cloud, Grid, Sun, Terrain};
use crate::visitor::Visitor;

pub struct DrawVisitor<'a> {
    canvas: &'a Painter3D,
    camera: &'a Camera,
    stroke: Stroke,
    background_color: Color32,
    mvp: Transform,
    sun: Option<&'a Sun>,
}

impl<'a> DrawVisitor<'a> {
    pub fn new(canvas: &'a Painter3D, camera: &'a Camera) -> Self {
        let resp_rect = canvas.resp_rect();
        let proj = camera.projection(resp_rect.width(), resp_rect.height());
        let camera_tf = proj * camera.view();

        canvas.rect(
            canvas.clip_rect().shrink(0.0),
            0.0,
            Color32::TRANSPARENT,
            Stroke::new(0.5, Color32::BLACK),
        );

        Self {
            canvas,
            camera,
            stroke: Stroke::new(1.0, Color32::GRAY),
            background_color: Color32::WHITE,
            mvp: Transform::new(camera_tf, resp_rect),
            sun: None,
        }
    }

    pub fn with_sun(mut self, sun: &'a Sun) -> Self {
        self.sun = Some(sun);
        self
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
    fn visit_camera(&self, _camera: &Camera) {
        debug!("Visit camera {:?}", self.camera);
    }

    #[allow(unused)]
    fn visit_cloud(&self, cloud: &Cloud) {
        // return;
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

        let mut img = egui::ColorImage::new([w, h], self.background_color);
        let background_color: Vec4 = self
            .background_color
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

        let sun_pos = self.sun.map(|x| x.get_pos()).unwrap_or_default();
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
                        if density > 0. {
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

                    let sky_col_base = col_a.lerp(col_b, ray_dir.y.clamp(0.0, 1.0).abs().sqrt());
                    let dst_fog = 0.5;
                    let sky = dst_fog * sky_col_base;
                    let background_color = background_color * (1.0 - dst_fog) + sky;

                    let focused_eye_cos = cos_angle
                        .clamp(-std::f32::consts::PI, std::f32::consts::PI)
                        .powf(cloud.params.x);
                    let sun = hg(focused_eye_cos, 0.4).clamp(-1.0, 1.0) * transmittance;

                    let cloud_col = light_energy * light_color.xyz();
                    let col = background_color.xyz() * transmittance + cloud_col;

                    *pixel = if transmittance >= 0.03 {
                        Color32::TRANSPARENT
                    } else {
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
            .load_texture("worley_texture", img, Default::default());
        let textureid = TextureId::from(&handle);
        self.canvas.image(
            textureid,
            egui::Rect::from_two_pos(min_tuple, max_tuple),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
        // self.visit_bounding_box(bb);
    }

    fn visit_grid(&self, grid: &Grid) {
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

    fn visit_bounding_box(&self, bb: &BoundingBox) {
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

    fn visit_sun(&self, sun: &Sun) {
        self.canvas.circle_filled(
            sun.get_pos(),
            sun.get_pos() + Vec3::new(0.1, 0.0, 0.0),
            Color32::LIGHT_YELLOW,
            self.mvp,
        );
    }

    fn visit_terrain(&self, terrain: &Terrain) {
        use rayon::prelude::*;

        let scl = terrain.scale as usize;
        let bb = terrain.bounding_box;
        let min = bb.min;
        let max = bb.max;

        terrain.triangle_strip.par_windows(3).for_each(|i| {
            let v0 = i[2] + Vec3::new(0.0, terrain.at(i[2].x, i[2].z), 0.0);
            let v1 = i[1] + Vec3::new(0.0, terrain.at(i[1].x, i[1].z), 0.0);
            let v2 = i[0] + Vec3::new(0.0, terrain.at(i[0].x, i[0].z), 0.0);

            let v0_xz = v0.xz();
            let v1_xz = v1.xz();
            let v2_xz = v2.xz();

            let angle = (v1_xz - v2_xz).angle_to(v0_xz - v2_xz);
            if (angle - std::f32::consts::FRAC_PI_4).abs() < 0.0001
                || (angle + std::f32::consts::FRAC_PI_2).abs() < 0.0001
            {
                let mut rng = thread_rng();
                let r = (rng.gen::<f32>() * 255.0).round() as u8;
                let g = (rng.gen::<f32>() * 255.0).round() as u8;
                let b = (rng.gen::<f32>() * 255.0).round() as u8;

                self.canvas
                    .triangle(v0, v1, v2, Color32::LIGHT_GREEN, self.mvp);
            }
        });
    }
}

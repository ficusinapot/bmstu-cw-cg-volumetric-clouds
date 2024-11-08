use std::cmp::max;

use cgmath::num_traits::real::Real;
use egui::{Color32, Pos2, Stroke, TextureId};
use glam::Vec3;
use log::debug;
use rand::random;

use crate::canvas::painter::Painter3D;
use crate::math::Transform;
use crate::object::camera::Camera;
use crate::object::objects::cloud::Cloud;
use crate::object::objects::{BoundingBox, Grid};
use crate::visitor::Visitor;

pub struct DrawVisitor<'a> {
    canvas: &'a Painter3D,
    camera: &'a Camera,
    stroke: Stroke,
    mvp: Transform,
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
            mvp: Transform::new(camera_tf, resp_rect),
        }
    }
}

impl<'a> Visitor for DrawVisitor<'a> {
    fn visit_camera(&self, _camera: &Camera) {
        debug!("Visit camera {:?}", self.camera);
    }

    fn visit_cloud(&self, cloud: &Cloud) {
        use rayon::prelude::*;

        let bb = cloud.bounding_box();
        let corners = bb.corners();

        let (width, height) = (1056.0, 900.0);

        let transformed = corners.iter().enumerate().map(|(i, &corner)| {
            self.canvas.transform(corner, self.mvp).unwrap_or(if i < 4 {
                Pos2::new(width, height)
            } else {
                Pos2::ZERO
            })
        });

        let (min_tuple, max_tuple) = transformed.fold(
            (Pos2::new(f32::MAX, f32::MAX), Pos2::new(f32::MIN, f32::MIN)),
            |(min, max), p| {
                (
                    Pos2::new(min.x.min(p.x), min.y.min(p.y)),
                    Pos2::new(max.x.max(p.x), max.y.max(p.y)),
                )
            },
        );

        let wh = max_tuple - min_tuple;
        let (w, h) = (wh.x as usize, wh.y as usize);

        let mut img = egui::ColorImage::new([w, h], Color32::TRANSPARENT);
        img.pixels.par_iter_mut().enumerate().for_each(|(idx, pixel)| {
            let i = idx / w;
            let j = idx % w; 

            *pixel = {

                
                let col = Color32::BLACK;
                let ray_origin = self.camera.get_position();
                let i = i + min_tuple.y as usize;
                let j = j + min_tuple.x as usize;
                let ray_dir = (self.camera.get_pixel_world_position(i, j, 1056, 900) - ray_origin)
                    .normalize();

                let ray_box_info = bb.dst(ray_origin, ray_dir);
                let dst_to_box = ray_box_info.x;
                let dst_inside_box = ray_box_info.y;


                let mut dst_travelled = 0.0;
                let dst_limit = dst_inside_box.min(f32::INFINITY - dst_to_box);
                let step_size = dst_inside_box / cloud.num_steps as f32;

                let mut transmittance = 1.0;
                let mut light_energy = Vec3::ZERO;

                let entry_point = ray_origin + dst_to_box * ray_dir;

                if dst_inside_box <= 0.0 {
                    Color32::TRANSPARENT
                } else {
                    while dst_travelled < dst_limit {
                        let ray_pos = entry_point + ray_dir * dst_travelled;
                        let density = cloud.sample_density(ray_pos);
                        if density > 0. {
                            // let light_transmittance = cloud.light_march(ray_pos);
                            // light_energy += density * step_size * transmittance * light_transmittance * phaze_val;
                            transmittance *= (-density * step_size).exp();
                            if transmittance < 0.01 {
                                break;
                            }
                        }
                        dst_travelled += step_size;
                    }

                    let (r, g, b, a) = col.to_tuple();

                    // let cloud_color = cloud.color;
                    let (r, g, b, a) = (r as f32, g as f32, b as f32, a as f32 * transmittance);

                    Color32::from_rgba_unmultiplied(r as u8, g as u8, b as u8, 255 - a as u8)
                }
            };
        });

        let handle = self
            .canvas
            .ctx()
            .load_texture("worley_texture", img, Default::default());
        let textureid = TextureId::from(&handle);
        self.canvas.image(
            textureid,
            egui::Rect::from_two_pos(min_tuple.into(), max_tuple.into()),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
        // self.visit_bounding_box(bb);
    }

    fn visit_grid(&self, grid: &Grid) {
        let k = grid.k;
        let scale = grid.scale;
        let stroke = self.stroke;

        let f = k as f32;
        for i in -k..=k {
            self.canvas.line(
                Vec3::new(-1., 0., i as f32 / f) * scale,
                Vec3::new(1., 0., i as f32 / f) * scale,
                stroke,
                self.mvp,
            );

            self.canvas.line(
                Vec3::new(i as f32 / f, 0., -1.) * scale,
                Vec3::new(i as f32 / f, 0., 1.) * scale,
                stroke,
                self.mvp,
            );
        }

        self.canvas.line(
            Vec3::new(0., 0., 0.) * scale,
            Vec3::new(0., 1., 0.) * scale,
            Stroke::new(2.0, Color32::DARK_GREEN),
            self.mvp,
        );

        self.canvas.line(
            Vec3::new(0., 0., 0.) * scale,
            Vec3::new(0., 0., 1.) * scale,
            Stroke::new(2.0, Color32::BLUE),
            self.mvp,
        );

        self.canvas.line(
            Vec3::new(0., 0., 0.) * scale,
            Vec3::new(1., 0., 0.) * scale,
            Stroke::new(2.0, Color32::RED),
            self.mvp,
        );

        self.canvas.text(
            Vec3::new(1., 0., 0.),
            egui::Align2::CENTER_BOTTOM,
            "x (1, 0, 0)",
            egui::FontId::monospace(14.0),
            Color32::BLACK,
            self.mvp,
        );

        self.canvas.text(
            Vec3::new(0., 1., 0.),
            egui::Align2::CENTER_BOTTOM,
            "y (0, 1, 0)",
            egui::FontId::monospace(14.0),
            Color32::BLACK,
            self.mvp,
        );

        self.canvas.text(
            Vec3::new(0., 0., 1.),
            egui::Align2::CENTER_BOTTOM,
            "z (0, 0, 1)",
            egui::FontId::monospace(14.0),
            Color32::BLACK,
            self.mvp,
        );

        // self.canvas.triangle(
        //     Vec3::new(0.0,0.0,0.0),
        //     Vec3::new(0.0,1.0,1.0),
        //     Vec3::new(1.0,1.0,1.0),
        //     Color32::RED,
        //     self.mvp
        // )
    }

    fn visit_bounding_box(&self, bb: &BoundingBox) {
        let corners = bb.corners();

        let edges = bb.edges();

        for &(i1, i2) in &edges {
            let (x1, y1, z1) = corners[i1].into();
            let (x2, y2, z2) = corners[i2].into();
            self.canvas.dashed_line(
                Vec3::new(x1, y1, z1),
                Vec3::new(x2, y2, z2),
                1.0,
                0.5,
                Stroke::new(1.0, Color32::LIGHT_GREEN),
                self.mvp,
            );
        }
    }
}

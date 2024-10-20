use egui::{Color32, Stroke};
use glam::Vec3;
use log::debug;
use crate::canvas::painter::Painter3D;
use crate::math::Transform;
use crate::object::camera::{Camera};
use crate::object::cloud::Cloud;
use crate::object::objects::Grid;
use crate::visitor::Visitor;

pub struct DrawVisitor<'a> {
    canvas: &'a Painter3D,
    camera: &'a Camera,
    stroke: Stroke
}

impl<'a> DrawVisitor<'a> {
    pub fn new(canvas: &'a Painter3D, camera: &'a Camera) -> Self {
        Self {
            canvas,
            camera,
            stroke: Stroke::new(1.0, Color32::GRAY)
        }
    }
}

impl<'a> Visitor for DrawVisitor<'a> {
    fn visit_camera(&self, _camera: &Camera) {
        debug!("Visit camera {:?}", self.camera);
        // debug!("Visit camera {:?}", self.canvas);
    }

    fn visit_cloud(&self, _cloud: &Cloud) {
        // self.scene.
    }

    fn visit_grid(&self, grid: &Grid) {

        let resp_rect = self.canvas.rect();
        let proj = self.camera.projection(resp_rect.width(), resp_rect.height());
        let camera_tf = proj * self.camera.view();

        let mvp = Transform::new(camera_tf, resp_rect);

        let k = grid.k;
        let scale = grid.scale;
        let stroke= self.stroke;
        
        let f = k as f32;
        for i in -k..=k {
            self.canvas.line(
                Vec3::new(-1., 0., i as f32 / f) * scale,
                Vec3::new(1., 0., i as f32 / f) * scale,
                stroke,
                mvp,
            );

            self.canvas.line(
                Vec3::new(i as f32 / f, 0., -1.) * scale,
                Vec3::new(i as f32 / f, 0., 1.) * scale,
                stroke,
                mvp,
            );
        }

        self.canvas.line(
            Vec3::new(0., 0., 0.) * scale,
            Vec3::new(0., 1.5, 0.) * scale,
            Stroke::new(2.0, Color32::BLUE),
            mvp,
        );

        self.canvas.line(
            Vec3::new(0., 0., 0.) * scale,
            Vec3::new(0., 0., -1.5) * scale,
            Stroke::new(2.0, Color32::RED),
            mvp,
        );

        self.canvas.line(
            Vec3::new(0., 0., 0.) * scale,
            Vec3::new(-1.5, 0., 0.) * scale,
            Stroke::new(2.0, Color32::DARK_GREEN),
            mvp,
        );
    }
}

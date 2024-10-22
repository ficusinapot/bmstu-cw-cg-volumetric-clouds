use crate::canvas::painter::Painter3D;
use crate::math::Transform;
use crate::object::camera::Camera;
use crate::object::objects::cloud::Cloud;
use crate::object::objects::{BoundingBox, Grid};
use crate::visitor::Visitor;
use egui::{Color32, Image, Rect, Stroke, TextureHandle, TextureId};
use glam::Vec3;
use log::debug;

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
        // debug!("{:?}", self.mvp.world_to_egui(Vec3::new(1.,0.,0.)));
        // debug!("{:?}", self.mvp.world_to_egui(Vec3::new(1., 1.,1.)));
        let bb = cloud.bounding_box();
        self.visit_bounding_box(bb);

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

        // let txt = self.canvas.ctx().load_texture(
        //     "test",
        //     egui::include_image!(),
        //     Default::default(),
        // )

        // self.canvas.bound_rect(corners[0], corners[4], TextureId::default(), self.mvp);

        // self.canvas.bound_rect(corners[0], corners[6], TextureId::default(), self.mvp);

        //
        self.canvas
            .bound_rect(corners[4], corners[5], TextureId::default(), self.mvp);
        self.canvas
            .bound_rect(corners[3], corners[7], TextureId::default(), self.mvp);
        // self.canvas.bound_rect(corners[5], corners[0], TextureId::default(), self.mvp);

        // self.canvas.bound_rect(corners[7], corners[2], TextureId::default(), self.mvp);

        let ray_origin = self.camera.get_pivot();
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
                Stroke::new(1.0, Color32::LIGHT_GREEN),
                self.mvp,
            );
        }
    }
}

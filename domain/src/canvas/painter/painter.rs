//! Extension to `egui` for 3D drawings

use egui::epaint::{Vertex, WHITE_UV};
use egui::{Color32, Pos2, Shape, Stroke, TextureId};
use std::ops::Deref;
// glam's types are part of our interface
// TODO: use mint? But then we'd have to convert every time ...
use crate::math::transform::Transform;
pub use glam;
pub use glam::Vec3;

#[derive(Clone)]
pub struct Painter3D {
    painter_2d: egui::Painter,
    resp_rect: egui::Rect,
}

impl Deref for Painter3D {
    type Target = egui::Painter;
    fn deref(&self) -> &Self::Target {
        &self.painter_2d
    }
}

impl Painter3D {
    pub fn new(painter_2d: egui::Painter, resp_rect: egui::Rect) -> Self {
        Self {
            painter_2d,
            resp_rect,
        }
    }

    pub fn resp_rect(&self) -> egui::Rect {
        self.resp_rect
    }

    pub fn text(
        &self,
        pos: Vec3,
        anchor: egui::Align2,
        text: impl ToString,
        font_id: egui::FontId,
        text_color: Color32,
        mvp: Transform,
    ) -> Option<egui::Rect> {
        self.transform(pos, mvp)
            .map(|pos| self.painter_2d.text(pos, anchor, text, font_id, text_color))
    }

    /// Transform a point in world coordinates to egui coordinates
    pub fn transform(&self, pt: Vec3, mvp: Transform) -> Option<egui::Pos2> {
        let (sc, z) = mvp.world_to_egui(pt);

        (0.0..=1.0).contains(&z).then(|| sc.to_pos2())
    }

    /// Get egui's 2D painter
    pub fn egui(&self) -> &egui::Painter {
        &self.painter_2d
    }

    // /// Returns a painter which has the given transformation prepended
    // pub fn prepend(&mut self, mat: Mat4) {
    //     self.transform.prepend(Transform::new(mat, self.resp_rect));
    // }
}

impl Painter3D {
    pub fn line(&self, a: Vec3, b: Vec3, stroke: Stroke, mvp: Transform) {
        let Some(a) = self.transform(a, mvp) else {
            return;
        };
        let Some(b) = self.transform(b, mvp) else {
            return;
        };
        self.painter_2d.line_segment([a, b], stroke);
    }

    pub fn dashed_line(
        &self,
        a: Vec3,
        b: Vec3,
        dash_length: f32,
        gap_length: f32,
        stroke: Stroke,
        mvp: Transform,
    ) {
        let Some(a) = self.transform(a, mvp) else {
            return;
        };
        let Some(b) = self.transform(b, mvp) else {
            return;
        };
        self.painter_2d
            .add(Shape::dashed_line(&[a, b], stroke, dash_length, gap_length));
    }

    pub fn bound_rect(
        &self,
        a: impl Into<Vec3>,
        b: impl Into<Vec3>,
        texture_id: TextureId,
        mvp: Transform,
    ) {
        let a = a.into();
        let b = b.into();

        let c = Vec3::new(a.x, b.y, a.z);
        let d = Vec3::new(b.x, a.y, b.z);

        println!("{:?} {:?} {:?} {:?}", a, b, c, d);

        let Some(a) = self.transform(a, mvp) else {
            return;
        };
        let Some(b) = self.transform(b, mvp) else {
            return;
        };
        let Some(c) = self.transform(c, mvp) else {
            return;
        };
        let Some(d) = self.transform(d, mvp) else {
            return;
        };

        let mut mesh = egui::Mesh::with_texture(texture_id);
        mesh.vertices.push(Vertex {
            pos: a,
            uv: WHITE_UV,
            color: Color32::RED,
        });
        mesh.vertices.push(Vertex {
            pos: b,
            uv: WHITE_UV,
            color: Color32::GREEN,
        });
        mesh.vertices.push(Vertex {
            pos: c,
            uv: WHITE_UV,
            color: Color32::BLUE,
        });
        mesh.vertices.push(Vertex {
            pos: d,
            uv: WHITE_UV,
            color: Color32::ORANGE,
        });

        mesh.add_triangle(0, 2, 3);
        mesh.add_triangle(1, 2, 3);
        self.painter_2d.add(mesh);
    }

    pub fn triangle(&self, a: Vec3, b: Vec3, c: Vec3, color32: Color32, mvp: Transform) {
        let Some(a) = self.transform(a, mvp) else {
            return;
        };
        let Some(b) = self.transform(b, mvp) else {
            return;
        };
        let Some(c) = self.transform(c, mvp) else {
            return;
        };

        let mut mesh = egui::Mesh::default();
        mesh.colored_vertex(a, color32);
        mesh.colored_vertex(b, color32);
        mesh.colored_vertex(c, color32);
        mesh.add_triangle(0, 1, 2);
        self.painter_2d.add(mesh);
    }

    // fn circle_filled(&self, center: Vec3, radius: f32, fill_color: impl Into<Color32>) {
    //     let Some(center) = self.transform(center) else {
    //         return;
    //     };
    //     self.painter_2d.circle_filled(center, radius, fill_color);
    // }
    //
    // fn circle(&self, center: Vec3, radius: f32, stroke: impl Into<Stroke>) {
    //     let Some(center) = self.transform(center) else {
    //         return;
    //     };
    //     self.painter_2d.circle_stroke(center, radius, stroke);
    // }
}

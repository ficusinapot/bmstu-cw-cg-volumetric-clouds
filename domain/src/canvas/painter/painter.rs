//! Extension to `egui` for 3D drawings
use egui::{Color32, Stroke};
// glam's types are part of our interface
// TODO: use mint? But then we'd have to convert every time ...
pub use glam;
pub use glam::Vec3;
use crate::math::transform::Transform;

#[derive(Clone)]
pub struct Painter3D {
    painter_2d: egui::Painter,
    resp_rect: egui::Rect,
}

impl Painter3D {
    pub fn new(painter_2d: egui::Painter, resp_rect: egui::Rect) -> Self {
        Self {
            painter_2d,
            resp_rect,
        }
    }

    pub fn rect(&self) -> egui::Rect {
        self.resp_rect
    }

    pub fn text(
        &self,
        pos: Vec3,
        anchor: egui::Align2,
        text: impl ToString,
        font_id: egui::FontId,
        text_color: Color32,
        mvp: Transform
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
        let Some(a) = self.transform(a, mvp) else { return };
        let Some(b) = self.transform(b, mvp) else { return };
        self.painter_2d.line_segment([a, b], stroke);
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
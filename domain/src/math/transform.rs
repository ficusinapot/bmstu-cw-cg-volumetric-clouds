// glam's types are part of our interface
// TODO: use mint? But then we'd have to convert every time ...
pub use glam;

use glam::{Mat4, Vec3Swizzles, Vec4Swizzles};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    rect: egui::Rect,
    mat: Mat4,
    inverse: Mat4,
}

impl Transform {
    pub fn new(mat: Mat4, rect: egui::Rect) -> Self {
        Self {
            rect,
            inverse: mat.inverse(),
            mat,
        }
    }

    /// Returns egui coordinates and z value for the given point
    pub fn world_to_egui(&self, world: glam::Vec3) -> (egui::Vec2, f32) {
        // World to "device coordinates"
        let pre: glam::Vec4 = self.mat * world.extend(1.);

        // Perspective division
        let mut dc = pre.xyz() / pre.w;

        // Invert Y
        dc.y *= -1.0;

        // Map to screen coordinates
        let sc = (dc + 1.) / 2.;
        let sc = sc.xy() * glam::Vec2::new(self.rect.width(), self.rect.height());

        let sc: mint::Vector2<f32> = sc.into();
        let sc: egui::Vec2 = sc.into();

        (sc + self.rect.min.to_vec2(), dc.z)
    }

    pub fn egui_to_world(&self, _egui: egui::Vec2, _z: f32) -> glam::Vec3 {
        /*
        let egui: mint::Vector2<f32> = egui.into();
        let egui: glam::Vec2 = egui.into();
        let egui = egui.extend(z);
        (self.inverse * egui.extend(1.)).xyz()
        */
        todo!()
    }

    /// Returns a Transform which has the given transformation prepended
    pub fn prepend(&mut self, tf: Transform) {
        self.mat = tf.mat * self.mat;
    }
}

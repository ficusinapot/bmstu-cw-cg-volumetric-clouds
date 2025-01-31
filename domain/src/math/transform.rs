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
        let pre: glam::Vec4 = self.mat * world.extend(1.);

        let mut dc = pre.xyz() / pre.w;

        dc.y *= -1.0;

        let sc = (dc + 1.) / 2.;
        let sc_glam = sc.xy() * glam::Vec2::new(self.rect.width(), self.rect.height());

        let sc: mint::Vector2<f32> = sc_glam.into();
        let sc: egui::Vec2 = sc.into();
        (sc + self.rect.min.to_vec2(), dc.z)
    }

    pub fn egui_to_world(&self, screen: egui::Vec2, depth: f32) -> glam::Vec3 {
        let sc = screen - self.rect.min.to_vec2();
        let sc =
            glam::Vec2::new(sc.x, sc.y) / glam::Vec2::new(self.rect.width(), self.rect.height());
        let mut dc = sc * 2.0 - glam::Vec2::ONE;

        dc.y *= -1.0;

        let ndc = glam::Vec3::new(dc.x, dc.y, depth).extend(1.0);

        let world = self.mat.inverse() * ndc;

        world.xyz() / world.w
    }

    /// Returns a Transform which has the given transformation prepended
    pub fn prepend(&mut self, tf: Transform) {
        self.mat = tf.mat * self.mat;
    }
}

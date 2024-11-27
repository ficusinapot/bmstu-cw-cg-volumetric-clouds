use cgmath::num_traits::Float;
use glam::{Vec3, Vec4, Vec4Swizzles, Mat4};
use log::info;

use crate::visitor::{Visitable, Visitor};

#[derive(Debug, Default, Copy, Clone)]
pub struct Sun {
    pos: Vec4,
    a: f32,
    d: f32
}

impl Sun {
    pub fn new(d: f32, a: f32) -> Self {
        let pos = Vec4::new(-1.0, 0.0, 0.0, 0.0);
        info!("Sun created at {:?}", pos);
        Self { pos, a, d }
    }

    #[inline]
    pub fn get_pos(&self) -> Vec3 {
        let mat = glam::Mat4::from_rotation_z(self.a.to_radians()) * glam::Mat4::from_scale(Vec3::splat(self.d));

        (mat * self.pos).xyz()
    }

    pub fn prepend_angle(&mut self, a: f32) {
        self.a = a
    }

    pub fn set_d(&mut self, d: f32) {
        self.d = d
    }
}

impl Visitable for Sun {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_sun(self);
    }
}

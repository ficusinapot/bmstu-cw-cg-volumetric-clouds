use glam::{Vec3, Vec4, Vec4Swizzles};

use crate::visitor::{Visitable, Visitor};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Sun {
    pos: Vec4,
    pub a: f32,
    pub z: f32,
    pub d: f32,
}

impl Sun {
    pub fn new(d: f32, a: f32, z: f32) -> Self {
        let pos = Vec4::new(-1.0, 0.0, 0.0, 0.0);
        Self { pos, a, z, d }
    }

    #[inline]
    pub fn get_pos(&self) -> Vec3 {
        let mat = glam::Mat4::from_rotation_y(self.z.to_radians()) * glam::Mat4::from_rotation_z(self.a.to_radians())
            * glam::Mat4::from_scale(Vec3::splat(self.d));
        (mat * self.pos).xyz()
    }

    pub fn prepend_angle(&mut self, a: glam::Vec2) {
        self.a = a.x;
        self.z = a.y;
    }
    

    pub fn set_d(&mut self, d: f32) {
        self.d = d
    }
}

impl Visitable for Sun {
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_sun(self);
    }
}

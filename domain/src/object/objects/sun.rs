use glam::{Vec3, Vec4, Vec4Swizzles};
use log::info;

use crate::visitor::{Visitable, Visitor};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Sun {
    pos: Vec4,
    pub a: f32,
    pub d: f32,
}

impl Sun {
    pub fn new(d: f32, a: f32) -> Self {
        let pos = Vec4::new(-1.0, 0.0, 0.0, 0.0);
        info!("Sun created at {:?}", pos);
        Self { pos, a, d }
    }

    pub fn calculate_sky_color(&self) -> egui::Color32 {
        let sun_angle = self.a.to_radians().clamp(0.0, 1.0);

        let horizon_color = Vec4::new(1.0, 0.6, 0.3, 1.0);
        let sky_color = Vec4::new(0.2, 0.6, 1.0, 1.0);
        let color = sky_color.lerp(horizon_color, 1.0 - sun_angle);

        egui::Color32::from_rgb(
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
        )
    }

    #[inline]
    pub fn get_pos(&self) -> Vec3 {
        let mat = glam::Mat4::from_rotation_z(self.a.to_radians())
            * glam::Mat4::from_scale(Vec3::splat(self.d));

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
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_sun(self);
    }
}

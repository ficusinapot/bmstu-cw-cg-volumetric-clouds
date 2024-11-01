use crate::object::objects::texture3d::Worley;
use crate::object::objects::BoundingBox;
use crate::visitor::{Visitable, Visitor};
use egui::Color32;
use glam::{Vec3, Vec4};

pub struct Cloud {
    bounding_box: BoundingBox,
    texture3d: Worley,
    pub clouds_offset: Vec3,
    pub cloud_scale: f32,
    pub density_threshold: f32,
    pub density_multiplier: f32,
    pub num_steps: usize,
    pub alpha_threshold: u8,
    pub color: Color32,
}

impl Cloud {
    pub fn set_clouds_offset(&mut self, offset: Vec3) {
        self.clouds_offset = offset;
    }

    pub fn set_cloud_scale(&mut self, scale: f32) {
        self.cloud_scale = scale;
    }

    pub fn set_density_threshold(&mut self, threshold: f32) {
        self.density_threshold = threshold;
    }

    pub fn set_density_multiplier(&mut self, multiplier: f32) {
        self.density_multiplier = multiplier;
    }

    pub fn set_alpha_threshold(&mut self, alpha_threshold: u8) {
        self.alpha_threshold = alpha_threshold;
    }
}

impl Cloud {
    pub fn new(bounding_box: impl Into<BoundingBox>) -> Self {
        Self {
            bounding_box: bounding_box.into(),
            texture3d: Worley::new(
                30,
                25,
                10,
                0,
                132,
                1.0,
                0.65,
                true,
                Vec4::new(1.0, 1.0, 1.0, 1.0),
            ),
            clouds_offset: Vec3::ZERO,
            cloud_scale: 69.0,
            density_threshold: 0.57,
            density_multiplier: 5.0,
            num_steps: 100,
            alpha_threshold: 255,
            color: Color32::WHITE,
        }
    }

    pub fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    pub fn sample_density(&self, position: Vec3) -> f32 {
        // let position = position + 0.5 * self.bounding_box.size();
        // let uvw = (position - self.bounding_box.min) / self.bounding_box.size();
        let uvw = (position) * self.cloud_scale * 0.001 + self.clouds_offset * 0.01;
        let shape = self.texture3d.get(uvw);

        0.0_f32.max(shape.x - self.density_threshold) * self.density_multiplier
    }
    
    pub fn light_march(&self, position: Vec3) -> f32 {
        0.0
    }
}

impl Visitable for Cloud {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_cloud(self);
    }
}

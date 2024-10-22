use crate::object::objects::texture3d::Texture3d;
use crate::object::objects::BoundingBox;
use crate::visitor::{Visitable, Visitor};
use glam::Vec3;

pub struct Cloud {
    bounding_box: BoundingBox,
    texture3d: Texture3d,
    clouds_offset: Vec3,
    cloud_scale: f32,
    density_threshold: f32,
    density_multiplier: f32,
}

impl Cloud {
    pub fn new(bounding_box: impl Into<BoundingBox>) -> Self {
        Self {
            bounding_box: bounding_box.into(),
            texture3d: Texture3d::new(),
            clouds_offset: Default::default(),
            cloud_scale: 0.0,
            density_threshold: 0.0,
            density_multiplier: 0.0,
        }
    }

    pub fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    pub fn sample_density(&self, position: Vec3) -> f32 {
        let uvw = position * self.cloud_scale * 0.001 + self.clouds_offset * 0.01;

        0.0
    }
}

impl Visitable for Cloud {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_cloud(self);
    }
}

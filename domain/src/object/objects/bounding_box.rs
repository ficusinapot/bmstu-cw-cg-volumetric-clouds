use crate::visitor::{Visitable, Visitor};
use cgmath::num_traits::real::Real;
use glam::Vec3;

#[derive(Debug)]
pub struct BoundingBox {
    /// One of the corners of the rectangle, usually the left top one.
    pub min: Vec3,

    /// The other corner, opposing [`Self::min`]. Usually the right bottom one.
    pub max: Vec3,
}

impl From<(Vec3, Vec3)> for BoundingBox {
    fn from(value: (Vec3, Vec3)) -> Self {
        Self::from_two_pos(value.0, value.1)
    }
}

impl BoundingBox {
    #[inline]
    pub fn from_two_pos(a: Vec3, b: Vec3) -> Self {
        Self {
            min: Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            max: Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        }
    }

    pub fn dst(&self, ray_origin: Vec3, ray_dir: Vec3) -> (f32, f32) {
        let t0 = (self.min - ray_origin) / ray_dir;
        let t1 = (self.max - ray_origin) / ray_dir;

        let tmin = t0.min(t1);
        let tmax = t0.max(t1);

        let dst_a = tmin.x.max(tmin.y).max(tmin.z);
        let dst_b = tmax.x.min(tmax.y).min(tmax.z);

        let dst_to_box = 0.0.max(dst_a);
        let dst_inside_box = 0.0.max(dst_b - dst_to_box);

        (dst_to_box, dst_inside_box)
    }
}

impl Visitable for BoundingBox {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_bounding_box(self)
    }
}

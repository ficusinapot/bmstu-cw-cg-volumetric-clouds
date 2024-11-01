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

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn center(&self) -> Vec3 {
        0.5 * (self.max + self.min)
    }

    pub fn contains(&self, position: Vec3) -> bool {
        (self.min.x..=self.max.x).contains(&position.x)
            && (self.min.y..=self.max.y).contains(&position.y)
            && (self.min.z..=self.max.z).contains(&position.z)
    }

    pub fn dst(&self, ray_origin: Vec3, ray_dir: Vec3) -> glam::Vec2 {
        let t0 = (self.min - ray_origin) / ray_dir;
        let t1 = (self.max - ray_origin) / ray_dir;

        let tmin = t0.min(t1);
        let tmax = t0.max(t1);

        let dst_a = tmin.x.max(tmin.y).max(tmin.z);
        let dst_b = tmax.x.min(tmax.y).min(tmax.z);

        let dst_to_box = 0.0.max(dst_a);
        let dst_inside_box = 0.0.max(dst_b - dst_to_box);

        (dst_to_box, dst_inside_box).into()
    }

    pub fn corners(&self) -> [Vec3; 8] {
        let (x1, y1, z1) = self.min.into();
        let (x2, y2, z2) = self.max.into();
        [
            Vec3::new(x1, y1, z1),
            Vec3::new(x2, y1, z1),
            Vec3::new(x1, y2, z1),
            Vec3::new(x1, y1, z2),
            Vec3::new(x2, y2, z1),
            Vec3::new(x2, y1, z2),
            Vec3::new(x1, y2, z2),
            Vec3::new(x2, y2, z2),
        ]
    }

    pub fn edges(&self) -> [(usize, usize); 12] {
        [
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 4),
            (2, 4),
            (1, 5),
            (3, 5),
            (2, 6),
            (3, 6),
            (4, 7),
            (6, 7),
            (5, 7),
        ]
    }
}

impl Visitable for BoundingBox {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_bounding_box(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bb() {
        let bb = BoundingBox::from_two_pos(Vec3::new(-1.0, 1.0, -1.0), Vec3::new(1.0, 1.5, 1.0));
        assert_eq!(
            glam::Vec2::new(4.0, 2.0),
            bb.dst(Vec3::new(5.0, 1.25, 0.0), Vec3::new(-1.0, 0.0, 0.0))
        );
    }
}

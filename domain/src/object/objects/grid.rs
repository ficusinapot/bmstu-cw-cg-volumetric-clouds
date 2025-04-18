use crate::visitor::{Visitable, Visitor};
pub use glam::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    pub k: i32,
    pub scale: f32,
}

impl Grid {
    pub fn new(k: i32, scale: f32) -> Self {
        Self { k, scale }
    }
}

impl Visitable for Grid {
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_grid(self);
    }
}

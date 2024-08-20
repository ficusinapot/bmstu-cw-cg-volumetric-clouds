use crate::visitor::{Visitable, Visitor};

pub trait Camera {}

#[derive(Default, Debug)]
pub struct FPSCamera {}

impl FPSCamera {
    pub fn new() -> Self {
        Self {}
    }
}

impl Camera for FPSCamera {}

impl Visitable for FPSCamera {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_camera(self);
    }
}

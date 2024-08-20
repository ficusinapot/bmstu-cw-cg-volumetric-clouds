use crate::scene::Component;
use crate::visitor::{Visitable, Visitor};

pub trait Camera {}

impl Component for dyn Camera {}

impl Visitable for dyn Camera {
    fn accept(&self, _visitor: &impl Visitor) {
        todo!()
    }
}

pub struct FPSCamera {}

impl FPSCamera {
    fn new() -> Self {
        Self {}
    }
}

impl Camera for FPSCamera {}

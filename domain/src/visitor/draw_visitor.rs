use crate::object::camera::FPSCamera;
use crate::scene::scene::Scene;
use crate::visitor::Visitor;
use log::debug;

pub struct DrawVisitor<'a> {
    scene: &'a Scene,
    camera: &'a FPSCamera,
}

impl<'a> DrawVisitor<'a> {
    pub fn new(scene: &'a Scene, camera: &'a FPSCamera) -> Self {
        Self { scene, camera }
    }
}

impl<'a> Visitor for DrawVisitor<'a> {
    fn visit_camera(&self, camera: &FPSCamera) {
        debug!("Visit camera {:?}", camera);
    }
}

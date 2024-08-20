use log::debug;
use crate::managers::Manager;
use crate::object::camera::FPSCamera;
use crate::scene::scene::Scene;
use crate::visitor::draw_visitor::DrawVisitor;
use crate::visitor::Visitable;

#[derive(Copy, Clone, Default, Debug)]
pub struct DrawManager {}

impl DrawManager {
    pub fn draw_scene(&self, camera: &FPSCamera, scene: &Scene) {
        debug!("{:?} {:?}", camera, scene);
        let visitor = DrawVisitor::new(scene, camera);
        scene.accept(&visitor);
    }
}

impl Manager for DrawManager {}

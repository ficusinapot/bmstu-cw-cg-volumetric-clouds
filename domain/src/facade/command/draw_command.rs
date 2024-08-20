use crate::facade::Command;
use crate::managers::camera_manager::CameraManager;
use log::debug;
use crate::managers::ManagerSolution;

#[derive(Debug)]
pub enum DrawCommandKind {
    Draw
}

#[derive(Debug)]
pub struct DrawCommand {
    kind: DrawCommandKind,
}

impl DrawCommand {
    pub fn new(kind: DrawCommandKind) -> Self {
        Self { kind }
    }
}

impl Command for DrawCommand {
    fn exec(&mut self, manager: &mut ManagerSolution) {
        debug!("Executing {:?}", self);
        match self.kind {
            DrawCommandKind::Draw => {
                let camera = manager.get_camera_manager().get_camera();
                let scene = manager.get_scene_manager().get_scene();
                manager.draw_manager.draw_scene(camera, scene);
            }
        }

    }
}

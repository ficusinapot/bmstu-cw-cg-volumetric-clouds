use crate::facade::Command;
use crate::managers::ManagerSolution;
use log::debug;

#[derive(Debug)]
pub enum DrawCommand {
    Draw,
}

impl Command for DrawCommand {
    fn exec(self, manager: &mut ManagerSolution) {
        debug!("Executing {:?}", self);
        match self {
            Self::Draw => {
                let camera = manager.get_camera_manager().get_camera();
                let scene = manager.get_scene_manager().get_scene();
                manager.draw_manager.draw_scene(camera, scene);
            }
        }
    }
}


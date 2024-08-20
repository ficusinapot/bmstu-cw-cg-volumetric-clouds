use crate::facade::Command;
use crate::managers::camera_manager::CameraManager;
use log::debug;
use crate::managers::ManagerSolution;

#[derive(Debug)]
pub enum CameraCommandKind {
    Rotate,
    Zoom,
}

#[derive(Debug)]
pub struct CameraCommand {
    kind: CameraCommandKind,
}

impl CameraCommand {
    pub fn new(kind: CameraCommandKind) -> Self {
        Self { kind }
    }
}

impl Command for CameraCommand {
    fn exec(&mut self, manager: &mut ManagerSolution) {
        debug!("Executing {:?}", manager);
        println!("Hello world!!!");
    }
}

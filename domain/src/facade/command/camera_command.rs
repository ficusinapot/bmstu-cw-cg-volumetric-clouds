use crate::facade::Command;
use crate::managers::camera_manager::CameraManager;
use log::debug;

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
    type CommandManager = CameraManager;
    fn exec(&mut self, manager: &mut CameraManager) {
        debug!("Executing {:?}", manager);
        println!("Hello world!!!");
    }
}

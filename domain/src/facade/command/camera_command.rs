use crate::facade::Command;
use crate::managers::ManagerSolution;
use log::debug;

#[derive(Debug)]
pub enum CameraCommandKind {
    Rotate,
    Zoom,
    Translate,
}

impl Command for CameraCommandKind {
    fn exec(self, manager: &mut ManagerSolution) {
        let command: CameraCommand = self.into();
        command.exec(manager);
    }
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
    fn exec(self, manager: &mut ManagerSolution) {
        debug!("Executing {:?}", manager);
        println!("Hello world!!!");
    }
}

impl From<CameraCommandKind> for CameraCommand {
    fn from(value: CameraCommandKind) -> Self {
        Self::new(value)
    }
}

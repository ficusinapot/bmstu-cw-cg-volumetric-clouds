use crate::facade::{Command};
use crate::managers::scene_manager::SceneManager;
use log::debug;
use crate::managers::ManagerSolution;
use crate::object::camera::FPSCamera;

#[derive(Debug)]
pub enum SceneCommandKind {
    AddObject,
    GetObject,
}

#[derive(Debug)]
pub struct SceneCommand {
    kind: SceneCommandKind
}

impl SceneCommand {
    pub fn new(kind: SceneCommandKind) -> Self {
        Self {
            kind
        }
    }
}

impl Command for SceneCommand {
    fn exec(&mut self, manager: &mut ManagerSolution) {
        debug!("Executing {:?}", self); 
        match self.kind {
            SceneCommandKind::AddObject => {
                debug!("add object");
                let object = FPSCamera::new();
                manager.scene_manager.add_object(object);
            }
            SceneCommandKind::GetObject => {
                debug!("get object");
            }
        }
    }
}

use crate::facade::Command;
use crate::managers::ManagerSolution;
use log::debug;

#[derive(Debug)]
pub enum SceneCommandKind {
    AddObject,
    GetObject,
    RemoveObject,
}

impl Command for SceneCommandKind {
    fn exec(self, manager: &mut ManagerSolution) {
        let command: SceneCommand = self.into();
        command.exec(manager);
    }
}

#[derive(Debug)]
pub struct SceneCommand {
    kind: SceneCommandKind,
}

impl SceneCommand {
    pub fn new(kind: SceneCommandKind) -> Self {
        Self { kind }
    }
}

impl Command for SceneCommand {
    fn exec(self, manager: &mut ManagerSolution) {
        debug!("Executing {:?}", self);
        match self.kind {
            SceneCommandKind::AddObject => {
                debug!("add object");
            }
            SceneCommandKind::GetObject => {
                debug!("get object");
            }
            SceneCommandKind::RemoveObject => {
                debug!("remove object");
            }
        }
    }
}

impl From<SceneCommandKind> for SceneCommand {
    fn from(value: SceneCommandKind) -> Self {
        Self::new(value)
    }
}

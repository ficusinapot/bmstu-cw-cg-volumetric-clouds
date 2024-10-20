use crate::facade::Command;
use crate::managers::ManagerSolution;
use log::debug;
use crate::object::Component;

pub enum SceneCommand {
    AddObject(Component),
    GetObject(Component),
    RemoveObject(Component),
}

impl Command for SceneCommand {
    fn exec(self, manager: &mut ManagerSolution) {
        match self {
            SceneCommand::AddObject(component) => {
                let sm = manager.get_mut_scene_manager();
                sm.add_object(component);
            }
            SceneCommand::GetObject(_component) => {
                debug!("get object");
            }
            SceneCommand::RemoveObject(_component) => {
                debug!("remove object");
            }
        }
    }
}

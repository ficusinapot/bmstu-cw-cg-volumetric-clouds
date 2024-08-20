use crate::facade::Command;
use crate::managers::object_manager::SceneManager;
use log::debug;

#[derive(Debug, Default)]
pub struct SceneCommand;

impl SceneCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for SceneCommand {
    type CommandManager = SceneManager;
    fn exec(&mut self, manager: &mut SceneManager) {
        debug!("Executing {:?}: {:?}", self, manager);
        manager.add_counter();
    }
}

use crate::facade::Command;
use crate::managers::ManagerSolution;

#[derive(Default)]
pub struct Facade {
    manager: ManagerSolution,
}

impl Facade {
    pub fn exec<C: Command>(&mut self, command: C) {
        command.exec(&mut self.manager);
    }
}

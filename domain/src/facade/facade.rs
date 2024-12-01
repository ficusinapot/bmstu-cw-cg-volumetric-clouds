use crate::facade::Command;
use crate::managers::ManagerSolution;

pub trait Executor {
    fn exec<C: Command>(&mut self, command: C) -> C::ReturnType;
}

#[derive(Default)]
pub struct Facade {
    manager: ManagerSolution,
}

impl Executor for Facade {
    fn exec<C: Command>(&mut self, command: C) -> C::ReturnType {
        command.exec(&mut self.manager)
    }
}

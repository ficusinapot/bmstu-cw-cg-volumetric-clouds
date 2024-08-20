use crate::facade::Command;
use crate::managers::ManagerSolution;

#[derive(Default)]
pub struct Facade {
    manager: ManagerSolution,
}

impl Facade {
    pub fn exec<C: Command>(&mut self, mut command: C)
    where
        for<'a> &'a mut <C as Command>::CommandManager: From<&'a mut ManagerSolution>,
    {
        let manager = &mut self.manager;
        command.exec(manager.into());
    }
}

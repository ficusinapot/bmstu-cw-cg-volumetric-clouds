use super::command::Command;

pub struct Facade;

impl Facade {
    pub fn exec(command: impl Command) {
        command.exec();
    }
}

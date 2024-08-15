use crate::facade::Command;
use log::debug;

#[derive(Debug, Default)]
pub struct ObjectCommand;

impl ObjectCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for ObjectCommand {
    fn exec(self) {
        debug!("Executing ObjectCommand");
        println!("Hello world!!!");
    }
}

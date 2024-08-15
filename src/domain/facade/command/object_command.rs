use crate::domain::facade::Command;

pub struct ObjectCommand;

impl ObjectCommand {
    pub fn new() -> Self {
        Self
    }
}

impl Command for ObjectCommand {
    fn exec(self) {
        println!("Hello world");
    }
}

mod object_command;

pub use object_command::ObjectCommand;

pub trait Command {
    fn exec(self);
}

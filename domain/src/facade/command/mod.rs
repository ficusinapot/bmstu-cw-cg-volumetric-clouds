mod camera_command;
mod object_command;

use crate::managers::Manager;
pub use camera_command::{CameraCommand, CameraCommandKind};
pub use object_command::SceneCommand;

pub trait Command {
    type CommandManager: Manager;

    fn exec(&mut self, manager: &mut Self::CommandManager);
}

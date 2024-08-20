mod camera_command;
mod scene_command;
mod draw_command;

use crate::managers::{ManagerSolution};
pub use camera_command::{CameraCommand, CameraCommandKind};
pub use scene_command::{SceneCommand, SceneCommandKind};
pub use draw_command::{DrawCommand, DrawCommandKind};

pub trait Command {

    fn exec(&mut self, manager: &mut ManagerSolution);
}

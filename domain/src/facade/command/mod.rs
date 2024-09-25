mod camera_command;
mod draw_command;
mod scene_command;

use crate::managers::ManagerSolution;
pub use camera_command::{CameraCommand, CameraCommandKind};
pub use draw_command::{DrawCommand};
pub use scene_command::{SceneCommand, SceneCommandKind};

pub trait Command: Sized {
    fn exec(self, manager: &mut ManagerSolution);
}

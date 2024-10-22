use crate::facade::Command;
use crate::managers::ManagerSolution;
use crate::object::camera::Camera;

#[derive(Debug)]
pub enum CameraCommand {
    Pan(f32, f32),
    Zoom(f32),
    Pivot(f32, f32),
    SetCamera(Camera),
}

impl Command for CameraCommand {
    type ReturnType = ();
    fn exec(self, manager: &mut ManagerSolution) {
        let cm = manager.get_mut_camera_manager();
        let camera = cm.get_mut_camera();
        match self {
            CameraCommand::Pan(x, y) => {
                camera.pan(x, y);
            }
            CameraCommand::Pivot(x, y) => {
                camera.pivot(x, y);
            }
            CameraCommand::Zoom(x) => {
                camera.zoom(x);
            }
            CameraCommand::SetCamera(c) => {
                cm.set_camera(c);
            }
        }
    }
}

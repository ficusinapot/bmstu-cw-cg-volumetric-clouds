use crate::managers::Manager;
use crate::object::camera::FPSCamera;

#[derive(Default, Debug)]
pub struct CameraManager {
    camera: FPSCamera
}

impl CameraManager {
    pub fn get_camera(&self) -> &FPSCamera {
        &self.camera
    }
}

impl Manager for CameraManager {}

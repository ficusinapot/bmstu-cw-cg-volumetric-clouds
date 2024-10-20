use crate::managers::Manager;
use crate::object::camera::Camera;

#[derive(Default, Debug)]
pub struct CameraManager {
    camera: Camera,
}

impl CameraManager {
    pub fn set_camera(&mut self, camera: Camera) { 
        self.camera = camera;
    }
    
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

impl Manager for CameraManager {}

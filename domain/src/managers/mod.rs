use crate::managers::camera_manager::CameraManager;
use crate::managers::object_manager::SceneManager;
use crate::managers::scene_manager::SceneManager;

pub mod camera_manager;
pub mod object_manager;
pub mod scene_manager;

pub trait Manager {}

#[derive(Default)]
pub struct ManagerSolution {
    scene_manager: SceneManager,
    camera_manager: CameraManager,
    object_manager: SceneManager,
}

impl<'a> From<&'a mut ManagerSolution> for &'a mut SceneManager {
    fn from(value: &'a mut ManagerSolution) -> &'a mut SceneManager {
        &mut value.scene_manager
    }
}

impl<'a> From<&'a mut ManagerSolution> for &'a mut SceneManager {
    fn from(value: &'a mut ManagerSolution) -> &'a mut SceneManager {
        &mut value.object_manager
    }
}

impl<'a> From<&'a mut ManagerSolution> for &'a mut CameraManager {
    fn from(value: &'a mut ManagerSolution) -> &'a mut CameraManager {
        &mut value.camera_manager
    }
}

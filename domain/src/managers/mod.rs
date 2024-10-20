use crate::managers::camera_manager::CameraManager;
use crate::managers::draw_manager::DrawManager;
use crate::managers::scene_manager::SceneManager;

pub mod camera_manager;
pub mod draw_manager;
pub mod scene_manager;

pub trait Manager {}

#[derive(Default)]
pub struct ManagerSolution {
    pub scene_manager: SceneManager,
    pub camera_manager: CameraManager,
    pub draw_manager: DrawManager,
}

impl ManagerSolution {
    #[inline]
    pub fn get_scene_manager(&self) -> &SceneManager {
        &self.scene_manager
    }

    #[inline]
    pub fn get_camera_manager(&self) -> &CameraManager {
        &self.camera_manager
    }

    #[inline]
    pub fn get_draw_manager(&self) -> &DrawManager {
        &self.draw_manager
    }

    #[inline]
    pub fn get_mut_scene_manager(&mut self) -> &mut SceneManager {
        &mut self.scene_manager
    }

    #[inline]
    pub fn get_mut_camera_manager(&mut self) -> &mut CameraManager {
        &mut self.camera_manager
    }

    #[inline]
    pub fn get_mut_draw_manager(&mut self) -> &mut DrawManager {
        &mut self.draw_manager
    }
}

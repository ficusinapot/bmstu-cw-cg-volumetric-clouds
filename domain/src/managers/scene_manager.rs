use crate::managers::Manager;
use crate::object::Component;
use crate::scene::scene::Scene;

#[derive(Debug, Default)]
pub struct SceneManager {
    scene: Scene
}

impl SceneManager {
    pub fn add_object(&mut self, object: impl Into<Component>) {
        self.scene.add_object(object);
    }
    
    pub fn get_scene(&self) -> &Scene {
        &self.scene
    }
}

impl Manager for SceneManager {}

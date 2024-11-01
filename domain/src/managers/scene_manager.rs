use crate::managers::Manager;
use crate::object::Component;
use crate::scene::scene::Scene;

#[derive(Default)]
pub struct SceneManager {
    scene: Scene,
}

impl SceneManager {
    pub fn add_object(&mut self, name: &'static str, object: impl Into<Component>) {
        self.scene.add_object(name, object);
    }

    pub fn get_object(&mut self, name: &'static str) {
        self.scene.get_object(name);
    }

    pub fn get_mut_object(&mut self, name: &'static str) -> Option<&mut Component> {
        self.scene.get_mut_object(name)
    }

    pub fn get_scene(&self) -> &Scene {
        &self.scene
    }
}

impl Manager for SceneManager {}

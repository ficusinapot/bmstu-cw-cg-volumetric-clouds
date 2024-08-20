use crate::managers::Manager;

#[derive(Copy, Clone, Default, Debug)]
pub struct SceneManager {
    pub counter: i32,
}

impl SceneManager {
    pub fn add(&mut self) {
        self.counter += 1;
    }
}

impl Manager for SceneManager {}

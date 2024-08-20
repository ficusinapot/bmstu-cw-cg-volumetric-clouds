use crate::managers::Manager;

#[derive(Copy, Clone, Default, Debug)]
pub struct SceneManager {
    count: i32
}

impl SceneManager {
    pub fn add_counter(&mut self) {
        self.count+=1;
    }
}

impl Manager for SceneManager {}

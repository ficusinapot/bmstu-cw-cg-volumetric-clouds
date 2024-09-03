pub mod camera;

use crate::object::camera::FPSCamera;
use crate::scene::scene_composite::SceneObjects;
use crate::visitor::{Visitable, Visitor};

#[derive(Debug)]
pub enum Component {
    Camera(FPSCamera),
    Composite(SceneObjects),
}

impl From<FPSCamera> for Component {
    fn from(value: FPSCamera) -> Self {
        Component::Camera(value)
    }
}

impl Visitable for Component {
    fn accept(&self, visitor: &impl Visitor) {
        match self {
            Component::Camera(camera) => camera.accept(visitor),
            Component::Composite(composite) => composite.accept(visitor),
        }
    }
}

pub mod camera;
pub mod objects;

use crate::object::camera::Camera;
use crate::object::objects::Grid;
use crate::scene::scene_composite::SceneObjects;
use crate::visitor::{Visitable, Visitor};
use objects::cloud::Cloud;

pub enum Component {
    Camera(Camera),
    Composite(SceneObjects),
    Cloud(Cloud),
    Grid(Grid),
}

impl From<Camera> for Component {
    fn from(value: Camera) -> Self {
        Component::Camera(value)
    }
}

impl From<Cloud> for Component {
    fn from(value: Cloud) -> Self {
        Component::Cloud(value)
    }
}

impl From<Grid> for Component {
    fn from(value: Grid) -> Self {
        Component::Grid(value)
    }
}

impl Visitable for Component {
    fn accept(&self, visitor: &impl Visitor) {
        match self {
            Component::Camera(camera) => camera.accept(visitor),
            Component::Composite(composite) => composite.accept(visitor),
            Component::Cloud(cloud) => cloud.accept(visitor),
            Component::Grid(grid) => grid.accept(visitor),
        }
    }
}

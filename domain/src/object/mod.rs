use glam::Vec3;
use objects::cloud::Cloud;

use crate::object::camera::Camera;
use crate::object::objects::{Grid, Sun, Terrain};
use crate::scene::scene_composite::SceneObjects;
use crate::visitor::{Visitable, Visitor};

pub mod camera;
pub mod objects;

#[derive(Debug)]
pub enum Component {
    Camera(Box<Camera>),
    Composite(SceneObjects),
    Cloud(Box<Cloud>),
    Sun(Sun),
    Grid(Grid),
    Terrain(Box<Terrain>),
}

impl Component {
    pub fn composite_from(
        it: impl IntoIterator<Item = (&'static str, impl Into<Component>)>,
    ) -> Self {
        let mut so = SceneObjects::default();
        for (name, obj) in it {
            so.add_object(name, obj.into());
        }
        Component::Composite(so)
    }

    pub fn pos(&self) -> glam::Vec3 {
        match self {
            Component::Camera(x) => x.pos(),
            Component::Composite(_) => Vec3::ZERO,
            Component::Cloud(x) => x.bounding_box.center(),
            Component::Sun(x) => x.get_pos(),
            Component::Grid(_) => Vec3::ZERO,
            Component::Terrain(x) => x.bounding_box.center(),
        }
    }
}

impl From<Camera> for Component {
    fn from(value: Camera) -> Self {
        Component::Camera(Box::new(value))
    }
}

impl From<Cloud> for Component {
    fn from(value: Cloud) -> Self {
        Component::Cloud(Box::new(value))
    }
}

impl From<Grid> for Component {
    fn from(value: Grid) -> Self {
        Component::Grid(value)
    }
}

impl From<Sun> for Component {
    fn from(value: Sun) -> Self {
        Component::Sun(value)
    }
}

impl From<Terrain> for Component {
    fn from(value: Terrain) -> Self {
        Component::Terrain(Box::new(value))
    }
}

impl Visitable for Component {
    fn accept(&self, visitor: &mut impl Visitor) {
        match self {
            Component::Camera(camera) => camera.accept(visitor),
            Component::Composite(composite) => composite.accept(visitor),
            Component::Cloud(cloud) => cloud.accept(visitor),
            Component::Grid(grid) => grid.accept(visitor),
            Component::Sun(sun) => sun.accept(visitor),
            Component::Terrain(ter) => ter.accept(visitor),
        }
    }
}

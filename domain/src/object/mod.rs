pub mod camera;
pub mod objects;

use crate::object::camera::Camera;
use crate::object::objects::{Grid, Sun};
use crate::scene::scene_composite::SceneObjects;
use crate::visitor::{Visitable, Visitor};
use objects::cloud::Cloud;
use crate::object::Component::Composite;

#[derive(Debug)]
pub enum Component {
    Camera(Camera),
    Composite(SceneObjects),
    Cloud(Box<Cloud>),
    Sun(Sun),
    Grid(Grid),
}

impl Component {
    pub fn composite_from(it: impl IntoIterator<Item = (&'static str, impl Into<Component>)>) -> Self {
        let mut so = SceneObjects::default();
        for (name, obj) in it {
            so.add_object(name, obj.into());
        }
        Composite(so)
    }
}

impl From<Camera> for Component {
    fn from(value: Camera) -> Self {
        Component::Camera(value)
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

impl Visitable for Component {
    fn accept(&self, visitor: &impl Visitor) {
        match self {
            Component::Camera(camera) => camera.accept(visitor),
            Component::Composite(composite) => composite.accept(visitor),
            Component::Cloud(cloud) => cloud.accept(visitor),
            Component::Grid(grid) => grid.accept(visitor),
            Component::Sun(sun) => sun.accept(visitor),
        }
    }
}

// impl IntoIterator for Component {
//     type Item = Component;
//     type IntoIter = Box<dyn Iterator<Item = Component>>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         match self {
//             Composite(scene_objects) => Box::new(scene_objects.into_iter()),
//             _ => Box::new(std::iter::once(self)),
//         }
//     }
// }
//
// impl IntoIterator for SceneObjects {
//     type Item = Component;
//     type IntoIter = IntoIter<Component>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         let x = self.objects.into_iter().map(|x| x.1).collect::<Vec<Self::Item>>();
//         x.into_iter()
//     }
// }

impl<'a> IntoIterator for &'a mut Component {
    type Item = &'a mut Component;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Composite(x) => x.objects.values_mut().collect::<Vec<_>>().into_iter(),
            _ => vec![self].into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iter() {
        let sun =Sun::new(10.0, -135.0);
        let mut sun = Some(Component::from(sun));
        for i in &mut sun {
            println!("1")
        }

        let mut comp = Component::composite_from(
            [("sun1", Component::from(Sun::new(10.0, -135.0))), ("sun2", Component::from(Sun::new(10.0, -135.0)))],
        );

        for i in &mut comp {
            println!("2")
        }
    }
}
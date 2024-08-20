use crate::scene::Component;
use crate::visitor::{Visitable, Visitor};
use std::collections::BTreeMap as Map;
use std::ops::{Deref, DerefMut};

// #[derive(Debug)]
pub struct SceneObjects {
    pub objects: Map<usize, Box<dyn Component>>,
    index: usize,
}

impl Component for SceneObjects {}

impl SceneObjects {
    pub fn add_object(&mut self, object: impl Component + 'static) -> Option<Box<dyn Component>> {
        self.index += 1;
        self.objects.insert(self.index, Box::new(object))
    }

    pub fn remove_object(&mut self, index: usize) -> Option<Box<dyn Component>> {
        self.objects.remove(&index)
    }
}

impl Deref for SceneObjects {
    type Target = Map<usize, Box<dyn Component>>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl DerefMut for SceneObjects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}

pub struct Scene {
    pub objects: SceneObjects,
}

impl Scene {
    pub fn add_object(&mut self, object: impl Component + 'static) -> Option<Box<dyn Component>> {
        self.objects.add_object(object)
    }

    pub fn remove_object(&mut self, index: usize) -> Option<Box<dyn Component>> {
        self.objects.remove_object(index)
    }

    pub fn get_object(&self, index: usize) -> Option<&Box<dyn Component>> {
        self.objects.get(&index)
    }
}

impl Visitable for Scene {
    fn accept(&self, visitor: &impl Visitor) {
        for i in self.objects.values() {
            i.accept(visitor)
        }
    }
}

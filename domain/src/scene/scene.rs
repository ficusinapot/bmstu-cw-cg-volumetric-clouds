use crate::facade::Command;
use crate::object::Component;
use crate::scene::scene_composite::SceneObjects;
use crate::visitor::{Visitable, Visitor};

#[derive(Default)]
pub struct Scene {
    pub objects: SceneObjects,
}

impl Scene {
    pub fn add_object(&mut self, name: &'static str, object: impl Into<Component>) {
        self.objects.add_object(name, object)
    }

    pub fn remove_object(&mut self, _index: usize) {
        unimplemented!()
    }

    pub fn get_object(&self, name: &'static str) -> Option<&Component> {
        self.objects.get_object(name)
    }

    pub fn get_mut_object(&mut self, name: &'static str) -> Option<&mut Component> {
        self.objects.get_mut_object(name)
    }
}

impl Visitable for Scene {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_composite(&self.objects);
    }
}

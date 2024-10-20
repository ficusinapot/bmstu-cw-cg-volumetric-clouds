use crate::object::Component;
use crate::scene::scene_composite::SceneObjects;
use crate::visitor::{Visitable, Visitor};

#[derive(Default)]
pub struct Scene {
    pub objects: SceneObjects,
}

impl Scene {
    pub fn add_object(&mut self, object: impl Into<Component>) {
        self.objects.add_object(object)
    }

    pub fn remove_object(&mut self, _index: usize) {
        unimplemented!()
    }

    pub fn get_object(&self, _index: usize) {
        unimplemented!()
    }
}

impl Visitable for Scene {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_composite(&self.objects);
    }
}

use crate::object::Component;
use crate::visitor::{Visitable, Visitor};
use std::collections::BTreeMap as Map;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct SceneObjects {
    pub objects: Map<usize, Component>,
    index: usize,
}

impl SceneObjects {
    pub fn add_object(&mut self, object: impl Into<Component>) {
        self.objects.insert(self.index, object.into());
        self.index += 1;
    }

    pub fn remove_object(&mut self, _index: usize) {
        unimplemented!()
    }

    pub fn get_object(&self, _index: usize) {
        unimplemented!()
    }
}

impl Visitable for SceneObjects {
    fn accept(&self, visitor: &impl Visitor) {
        for i in self.objects.values() {
            i.accept(visitor);
        }
    }
}

impl Deref for SceneObjects {
    type Target = Map<usize, Component>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl DerefMut for SceneObjects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}

use std::collections::BTreeMap as Map;
use std::ops::{Deref, DerefMut};
use crate::object::Component;
use crate::visitor::{Visitable, Visitor};

#[derive(Debug, Default)]
pub struct SceneObjects {
    pub objects: Map<usize, Component>,
    index: usize,
}

impl SceneObjects {
    pub fn add_object(&mut self, object: impl Into<Component>) {
        self.objects.insert(self.index, object.into());
    }

    pub fn remove_object(&mut self, index: usize) {
        unimplemented!()
    }

    pub fn get_object(&self, index: usize)  {
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
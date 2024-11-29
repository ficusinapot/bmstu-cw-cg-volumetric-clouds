use crate::object::Component;
use crate::visitor::{Visitable, Visitor};
use std::collections::BTreeMap as Map;
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SceneObjects {
    pub objects: Map<&'static str, Component>,
}

impl SceneObjects {
    pub fn add_object(&mut self, name: &'static str, object: impl Into<Component>) {
        self.objects.insert(name, object.into());
    }

    pub fn remove_object(&mut self, _index: usize) {
        unimplemented!()
    }

    pub fn get_object(&self, name: &'static str) -> Option<&Component> {
        self.objects.get(name)
    }

    pub fn get_mut_object(&mut self, name: &'static str) -> Option<&mut Component> {
        self.objects.get_mut(name)
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
    type Target = Map<&'static str, Component>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl DerefMut for SceneObjects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}

use crate::visitor::{Visitable, Visitor};

#[allow(clippy::module_inception)]
pub mod scene;

pub trait Component {}

impl Visitable for dyn Component {
    fn accept(&self, _visitor: &impl Visitor) {
        todo!()
    }
}

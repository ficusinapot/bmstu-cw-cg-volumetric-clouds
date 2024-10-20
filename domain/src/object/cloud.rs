use crate::visitor::{Visitable, Visitor};

#[derive(Debug)]
pub struct Cloud {}

impl Visitable for Cloud {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_cloud(self)
    }
}
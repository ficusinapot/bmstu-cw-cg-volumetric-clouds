pub trait Visitable {
    fn accept(&self, visitor: &impl Visitor);
}

pub trait Visitor {}

use crate::object::camera::FPSCamera;
use crate::scene::scene_composite::SceneObjects;

pub mod draw_visitor;

pub trait Visitable {
    fn accept(&self, visitor: &impl Visitor);
}

pub trait Visitor: Sized {
    fn visit_composite(&self, scene_objects: &SceneObjects) {
        for i in scene_objects.values() {
            i.accept(self)
        }
    }
    
    fn visit_camera(&self, _camera: &FPSCamera) {}
}

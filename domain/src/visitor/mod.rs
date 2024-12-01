use crate::object::camera::Camera;
use crate::object::objects::cloud::Cloud;
use crate::object::objects::{BoundingBox, Grid, Sun, Terrain};
use crate::scene::scene_composite::SceneObjects;

pub mod draw_visitor;

pub trait Visitable {
    fn accept(&self, visitor: &mut impl Visitor);
}

pub trait Visitor: Sized + Send + Sync {
    fn visit_composite(&mut self, scene_objects: &SceneObjects) {
        // use rayon::prelude::*;
        for (_, i) in scene_objects.iter() {
            i.accept(self)
        }
    }

    fn visit_camera(&mut self, _camera: &Camera) {}
    fn visit_cloud(&mut self, _cloud: &Cloud) {}
    fn visit_grid(&mut self, _grid: &Grid) {}
    fn visit_bounding_box(&mut self, _bb: &BoundingBox) {}
    fn visit_sun(&mut self, _bb: &Sun) {}

    fn visit_terrain(&mut self, _terrain: &Terrain) {}
}

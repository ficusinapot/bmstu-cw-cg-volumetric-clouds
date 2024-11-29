use crate::object::camera::Camera;
use crate::object::objects::cloud::Cloud;
use crate::object::objects::{BoundingBox, Grid, Sun, Terrain};
use crate::scene::scene_composite::SceneObjects;

pub mod draw_visitor;

pub trait Visitable {
    fn accept(&self, visitor: &impl Visitor);
}

pub trait Visitor: Sized + Send + Sync {
    fn visit_composite(&self, scene_objects: &SceneObjects) {
        use rayon::prelude::*;
        scene_objects.par_iter().for_each(|(_, x)| x.accept(self))
    }

    fn visit_camera(&self, _camera: &Camera) {}
    fn visit_cloud(&self, _cloud: &Cloud) {}
    fn visit_grid(&self, _grid: &Grid) {}
    fn visit_bounding_box(&self, _bb: &BoundingBox) {}
    fn visit_sun(&self, _bb: &Sun) {}

    fn visit_terrain(&self, _terrain: &Terrain) {}
}

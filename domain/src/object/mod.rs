use objects::cloud::Cloud;

use crate::object::camera::Camera;
use crate::object::objects::{Grid, Sun, Terrain};
use crate::scene::scene_composite::SceneObjects;
use crate::visitor::{Visitable, Visitor};

pub mod camera;
pub mod objects;

#[derive(Debug, PartialEq, Clone)]
pub enum Component {
    Camera(Camera),
    Composite(SceneObjects),
    Cloud(Box<Cloud>),
    Sun(Sun),
    Grid(Grid),
    Terrain(Terrain),
}

impl Component {
    pub fn composite_from(
        it: impl IntoIterator<Item = (&'static str, impl Into<Component>)>,
    ) -> Self {
        let mut so = SceneObjects::default();
        for (name, obj) in it {
            so.add_object(name, obj.into());
        }
        Component::Composite(so)
    }
}

impl From<Camera> for Component {
    fn from(value: Camera) -> Self {
        Component::Camera(value)
    }
}

impl From<Cloud> for Component {
    fn from(value: Cloud) -> Self {
        Component::Cloud(Box::new(value))
    }
}

impl From<Grid> for Component {
    fn from(value: Grid) -> Self {
        Component::Grid(value)
    }
}

impl From<Sun> for Component {
    fn from(value: Sun) -> Self {
        Component::Sun(value)
    }
}

impl From<Terrain> for Component {
    fn from(value: Terrain) -> Self {
        Component::Terrain(value)
    }
}

impl Visitable for Component {
    fn accept(&self, visitor: &impl Visitor) {
        match self {
            Component::Camera(camera) => camera.accept(visitor),
            Component::Composite(composite) => composite.accept(visitor),
            Component::Cloud(cloud) => cloud.accept(visitor),
            Component::Grid(grid) => grid.accept(visitor),
            Component::Sun(sun) => sun.accept(visitor),
            Component::Terrain(ter) => ter.accept(visitor),
        }
    }
}

impl<'a> IntoIterator for &'a mut Component {
    type Item = &'a mut Component;
    type IntoIter = Box<dyn Iterator<Item = &'a mut Component> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Component::Composite(x) => Box::new(x.objects.values_mut()),
            _ => Box::new(std::iter::once(self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use egui::Color32;
    use glam::{Vec3, Vec4};

    use crate::object::objects::cloud::CloudBuilder;

    use super::*;

    #[test]
    fn test_iter() {
        let cloud = CloudBuilder::default()
            .with_bounding_box((Vec3::new(-2.5, 1., -2.5), Vec3::new(2.5, 1.8, 2.5)))
            .with_shape_offset(Vec3::ZERO)
            .with_detail_offset(Vec3::ZERO)
            .with_cloud_scale(290.0)
            .with_density_threshold(0.95)
            .with_density_multiplier(3600.0)
            .with_num_steps(200)
            .with_num_steps_light(20)
            .with_density_offset(-8.30)
            .with_shape_noise_weights(Vec4::new(3.0, 6.0, 5.0, 1.0))
            .with_detail_noise_weight(1.0)
            .with_detail_weights(Vec4::new(4.0, 1.5, 1.5, 3.0))
            .with_detail_noise_scale(1.09)
            .with_color(Color32::WHITE)
            .with_col_a(Color32::WHITE)
            .with_col_b(Color32::LIGHT_BLUE)
            .with_light_color(Color32::WHITE)
            .with_light_absorption_through_cloud(0.7)
            .with_light_absorption_toward_sun(0.6)
            .with_phase_params(Vec4::new(0.05, 0.48, 0.37, 0.99))
            .with_darkness_threshold(0.35)
            .with_edge_distance(1.0)
            .with_ray_offset_strength(0.0)
            .with_volume_offset(0.0)
            .with_height_map_factor(2.0)
            .with_clouds_offset(Vec3::new(0.0, 0.0, 0.0))
            .build();
        let expected = cloud.clone();
        let cloud1 = cloud.clone();
        let mut cloud = Some(Component::from(cloud));

        let mut result = Vec::new();
        for i in cloud.iter_mut() {
            result.push(i.clone());
        }
        assert_eq!(result, vec![Component::from(expected.clone())]);

        let sun = Component::from(Sun::new(10.0, -135.0)).clone();
        let cloud = Component::from(cloud1.clone());
        let expected_sun = sun.clone();
        let expected_cloud = cloud.clone();
        let mut comp = Component::composite_from([("sun", sun), ("cloud", cloud)]);

        let mut result = Vec::new();
        for i in comp.into_iter() {
            result.push(i.clone());
        }
        assert!(
            result == vec![expected_sun.clone(), expected_cloud.clone()]
                || result == vec![expected_cloud, expected_sun]
        );
    }
}

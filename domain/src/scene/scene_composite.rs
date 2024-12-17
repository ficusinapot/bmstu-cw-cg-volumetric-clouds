use std::collections::BTreeMap as Map;
use std::ops::{Deref, DerefMut};

use crate::object::Component;
use crate::visitor::{Visitable, Visitor};

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
    fn accept(&self, visitor: &mut impl Visitor) {
        for i in self.objects.values().map(|x| x) {
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

#[cfg(test)]
mod tests {
    use egui::Color32;
    use glam::{Vec3, Vec4};

    use crate::object::objects::cloud::CloudBuilder;
    use crate::object::objects::terrain::TerrainBuilder;
    use crate::object::objects::texture3d::{PerlinBuilder, WorleyBuilder};
    use crate::object::objects::{Cloud, Sun};

    use super::*;

    #[test]
    fn test_add_object() {
        let mut scene = SceneObjects::default();
        let terrain_params = TerrainBuilder::default()
            .with_bounding_box((Vec3::new(-2.1, 0.0, -2.5), Vec3::new(2.5, 0.5, 2.5)))
            .with_scale(65)
            .with_bottom_color(Color32::from_rgb(181, 255, 182))
            .with_top_color(Color32::from_rgb(134, 167, 134))
            .with_density_scale(50.0)
            .with_diffuse_factor(0.55)
            .with_num_shadows_steps(5)
            .with_shadow_threshold(0.6)
            .with_noise(
                PerlinBuilder::new()
                    .with_seed(1)
                    .with_num_points_a(1)
                    .with_num_points_b(2)
                    .with_num_points_c(5)
                    .with_tile(1.0)
                    .with_resolution(64)
                    .with_color_mask(Vec4::new(1.0, 1.0, 1.0, 1.0))
                    .with_persistence(1.3)
                    .with_invert_noise(true),
            );
        let component: Component = terrain_params.build().into();
        scene.add_object("terrain", component.clone());

        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.get_object("terrain"), Some(&component));
    }

    #[test]
    fn test_get_object() {
        let mut scene = SceneObjects::default();
        let component = Component::Sun(Sun::default());
        scene.add_object("object1", component.clone());

        let retrieved = scene.get_object("object1");
        assert_eq!(retrieved, Some(&component));
    }

    #[test]
    fn test_get_mut_object() {
        let mut scene = SceneObjects::default();
        let component = Component::Sun(Sun::new(0.0, 0.0));
        scene.add_object("object1", component.clone());

        if let Some(retrieved) = scene.get_mut_object("object1") {
            if let Component::Sun(sun) = retrieved {
                sun.prepend_angle(10.0)
            }
        }

        let modified_component = Component::Sun(Sun::new(0.0, 10.0));
        assert_eq!(scene.get_object("object1"), Some(&modified_component));
    }

    #[test]
    fn test_deref() {
        let mut scene = SceneObjects::default();
        scene.add_object("object1", Component::Sun(Sun::new(10.0, 5.0)));
        scene.add_object("object2", Component::Sun(Sun::new(10.0, 5.0)));

        let object_count = scene.len();
        assert_eq!(object_count, 2);
    }

    #[test]
    #[should_panic(expected = "cloud")]
    fn test_accept_visitor() {
        struct TestVisitor;
        impl Visitor for TestVisitor {
            fn visit_composite(&mut self, scene_objects: &SceneObjects) {
                for i in &scene_objects.objects {
                    i.1.accept(self)
                }
            }
            fn visit_cloud(&mut self, _cloud: &Cloud) {
                _cloud.sample_density(Vec3::ZERO);
                _cloud.light_march(Vec3::ZERO, Vec3::ONE);
                panic!("cloud")
            }
        }

        let mut scene = SceneObjects::default();

        let noise = WorleyBuilder::new()
            .with_seed(2)
            .with_num_points_a(6)
            .with_num_points_b(12)
            .with_num_points_c(22)
            .with_tile(1.0)
            .with_resolution(128)
            .with_color_mask(Vec4::new(0.9, 1.0, 1.0, 1.0))
            .with_persistence(0.84)
            .with_invert_noise(true);

        let detail_noise = WorleyBuilder::new()
            .with_seed(1)
            .with_num_points_a(7)
            .with_num_points_b(7)
            .with_num_points_c(11)
            .with_tile(1.0)
            .with_resolution(64)
            .with_color_mask(Vec4::new(1.0, 1.0, 1.0, 1.0))
            .with_persistence(0.89)
            .with_invert_noise(true);

        let cloud_params = CloudBuilder::default()
            .with_map_size(glam::IVec3::ZERO)
            .with_bounding_box((Vec3::new(-1.5, 1.9, -3.5), Vec3::new(3.5, 2.5, 3.5)))
            .with_shape_offset(Vec3::ZERO)
            .with_detail_offset(Vec3::ZERO)
            .with_cloud_scale(290.0)
            .with_density_threshold(0.95)
            .with_density_multiplier(360.0)
            .with_num_steps(130)
            .with_num_steps_light(20)
            .with_density_offset(-9.30)
            .with_noise(noise)
            .with_shape_noise_weights(Vec4::new(3.0, 6.0, 5.0, 1.0))
            .with_detail_noise(detail_noise)
            .with_detail_noise_weight(1.0)
            .with_detail_weights(Vec4::new(4.0, 1.5, 1.5, 3.0))
            .with_detail_noise_scale(1.09)
            .with_color(Color32::WHITE)
            .with_col_a(Color32::WHITE)
            .with_col_b(Color32::LIGHT_BLUE)
            .with_light_color(Color32::WHITE)
            .with_light_absorption_through_cloud(0.6)
            .with_light_absorption_toward_sun(0.55)
            .with_phase_params(Vec4::new(0.00, 0.48, 0.37, 0.99))
            .with_darkness_threshold(0.35)
            .with_edge_distance(1.0)
            .with_ray_offset_strength(0.0)
            .with_volume_offset(0.0)
            .with_height_map_factor(2.0)
            .with_clouds_offset(Vec3::new(0.0, 0.0, 0.0))
            .with_weather_noise(
                PerlinBuilder::new()
                    .with_num_points_a(12)
                    .with_num_points_b(2)
                    .with_num_points_c(4)
                    .with_tile(1.0)
                    .with_resolution(128)
                    .with_color_mask(Vec4::new(1.0, 1.0, 1.0, 1.0))
                    .with_persistence(0.8)
                    .with_invert_noise(false),
            );

        let cloud = cloud_params.build();
        let component: Component = cloud.into();
        scene.add_object("cloud", component);
        let mut visitor = TestVisitor;
        scene.accept(&mut visitor);

        assert_eq!(scene.len(), 1);
    }
}

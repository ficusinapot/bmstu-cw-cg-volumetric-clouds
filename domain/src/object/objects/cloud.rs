use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

use crate::object::objects::texture3d::{INoise, INoiseBuilder, Noise, NoiseBuilder};
use crate::visitor::{Visitable, Visitor};
use egui::Color32;
use glam::{FloatExt, IVec3, Vec3, Vec3Swizzles, Vec4};
use log::info;

use super::BoundingBox;

#[inline]
pub fn remap(v: f32, min_old: f32, max_old: f32, min_new: f32, max_new: f32) -> f32 {
    min_new + (v - min_old) * (max_new - min_new) / (max_old - min_old)
}

#[inline]
pub fn hg(a: f32, g: f32) -> f32 {
    let g2 = g * g;
    (1.0 - g2) / (4.0 * std::f32::consts::PI * (1.0 + g2 - 2.0 * g * a).powf(1.5))
}

#[inline]
pub fn phase(a: f32, phase_params: Vec4) -> f32 {
    let blend = 0.5;
    let hg_blend = hg(a, phase_params.x) * (1.0 - blend) + hg(a, phase_params.y) * blend;
    phase_params.y + hg_blend * phase_params.z
}

#[inline]
pub fn beer(d: f32) -> f32 {
    (-d).exp()
}

#[derive(Default, Debug, Copy,Clone)]
pub struct CloudBuilder {
    pub bounding_box: BoundingBox,
    pub offset: Vec3,
    pub cloud_scale: f32,
    pub density_threshold: f32,
    pub density_offset: f32,
    pub density_multiplier: f32,

    pub num_steps_light: usize,
    pub num_steps: usize,
    pub ray_offset_strength: f32,

    pub alpha_threshold: u8,
    pub color: Color32,

    pub params: Vec4,
    pub map_size: IVec3,
    pub detail_noise_scale: f32,
    pub detail_noise_weight: f32,
    pub detail_weights: Vec4,
    pub shape_noise_weights: Vec4,
    pub phase_params: Vec4,

    pub shape_offset: Vec3,
    pub detail_offset: Vec3,

    pub light_absorption_toward_sun: f32,
    pub light_absorption_through_cloud: f32,
    pub darkness_threshold: f32,
    pub light_color: Color32,
    pub col_a: Color32,
    pub col_b: Color32,
    pub noise: NoiseBuilder,
    pub detail_noise: NoiseBuilder,
    pub weather_noise: NoiseBuilder,
    pub height_map_factor: f32,
    pub volume_offset: f32,
    pub edge_distance: f32,
}

impl CloudBuilder {
    pub fn build(self) -> Cloud {
        Cloud::build(self)
    }

    pub fn with_bounding_box(mut self, bounding_box: impl Into<BoundingBox>) -> Self {
        self.bounding_box = bounding_box.into();
        self
    }

    pub fn with_noise(mut self, builder: impl Into<NoiseBuilder>) -> Self {
        self.noise = builder.into();
        self
    }

    pub fn with_detail_noise(mut self, builder: impl Into<NoiseBuilder>) -> Self {
        self.detail_noise = builder.into();
        self
    }

    pub fn with_weather_noise(mut self, builder: impl Into<NoiseBuilder>) -> Self {
        self.weather_noise = builder.into();
        self
    }

    pub fn with_clouds_offset(mut self, clouds_offset: Vec3) -> Self {
        self.offset = clouds_offset;
        self
    }

    pub fn with_cloud_scale(mut self, cloud_scale: f32) -> Self {
        self.cloud_scale = cloud_scale;
        self
    }

    pub fn with_density_threshold(mut self, density_threshold: f32) -> Self {
        self.density_threshold = density_threshold;
        self
    }

    pub fn with_density_multiplier(mut self, density_multiplier: f32) -> Self {
        self.density_multiplier = density_multiplier;
        self
    }

    pub fn with_density_offset(mut self, density_offset: f32) -> Self {
        self.density_offset = density_offset;
        self
    }

    pub fn with_ray_offset_strength(mut self, ray_offset_strength: f32) -> Self {
        self.ray_offset_strength = ray_offset_strength;
        self
    }

    pub fn with_num_steps_light(mut self, num_steps: usize) -> Self {
        self.num_steps_light = num_steps;
        self
    }

    pub fn with_num_steps(mut self, num_steps: usize) -> Self {
        self.num_steps = num_steps;
        self
    }

    pub fn with_alpha_threshold(mut self, alpha_threshold: u8) -> Self {
        self.alpha_threshold = alpha_threshold;
        self
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn with_map_size(mut self, map_size: IVec3) -> Self {
        self.map_size = map_size;
        self
    }

    pub fn with_detail_noise_scale(mut self, detail_noise_scale: f32) -> Self {
        self.detail_noise_scale = detail_noise_scale;
        self
    }

    pub fn with_detail_noise_weight(mut self, detail_noise_weight: f32) -> Self {
        self.detail_noise_weight = detail_noise_weight;
        self
    }

    pub fn with_detail_weights(mut self, detail_weights: Vec4) -> Self {
        self.detail_weights = detail_weights;
        self
    }

    pub fn with_shape_noise_weights(mut self, shape_noise_weights: Vec4) -> Self {
        self.shape_noise_weights = shape_noise_weights;
        self
    }

    pub fn with_phase_params(mut self, phase_params: Vec4) -> Self {
        self.phase_params = phase_params;
        self
    }

    pub fn with_shape_offset(mut self, shape_offset: Vec3) -> Self {
        self.shape_offset = shape_offset;
        self
    }

    pub fn with_detail_offset(mut self, detail_offset: Vec3) -> Self {
        self.detail_offset = detail_offset;
        self
    }

    pub fn with_light_absorption_toward_sun(mut self, light_absorption_toward_sun: f32) -> Self {
        self.light_absorption_toward_sun = light_absorption_toward_sun;
        self
    }

    pub fn with_light_absorption_through_cloud(
        mut self,
        light_absorption_through_cloud: f32,
    ) -> Self {
        self.light_absorption_through_cloud = light_absorption_through_cloud;
        self
    }

    pub fn with_darkness_threshold(mut self, darkness_threshold: f32) -> Self {
        self.darkness_threshold = darkness_threshold;
        self
    }

    pub fn with_light_color(mut self, light_color: Color32) -> Self {
        self.light_color = light_color;
        self
    }

    pub fn with_col_a(mut self, col_a: Color32) -> Self {
        self.col_a = col_a;
        self
    }

    pub fn with_col_b(mut self, col_b: Color32) -> Self {
        self.col_b = col_b;
        self
    }

    pub fn with_height_map_factor(mut self, height_map_factor: f32) -> Self {
        self.height_map_factor = height_map_factor;
        self
    }

    pub fn with_volume_offset(mut self, volume_offset: f32) -> Self {
        self.volume_offset = volume_offset;
        self
    }

    pub fn with_edge_distance(mut self, edge_distance: f32) -> Self {
        self.edge_distance = edge_distance;
        self
    }
}

#[derive(Clone, Default)]
pub struct Cloud {
    noise: Noise,
    detail_noise: Noise,
    weather_map: Noise,
    pub cloud_params: CloudBuilder,
}

impl Debug for Cloud {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cloud")
    }
}

impl Deref for Cloud {
    type Target = CloudBuilder;
    fn deref(&self) -> &Self::Target {
        &self.cloud_params
    }
}

impl DerefMut for Cloud {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cloud_params
    }
}

impl Cloud {
    pub fn build(cloud_params: CloudBuilder) -> Self {
        info!("Cloud created at {:?}", cloud_params.bounding_box);
        let noise = cloud_params.noise.build();
        let detail_noise = cloud_params.detail_noise.build();
        let weather_map = cloud_params.weather_noise.build();
        Self {
            cloud_params,
            noise,
            detail_noise,
            weather_map,
        }
    }

    pub fn regenerate_noise(&mut self, builder: impl Into<NoiseBuilder>) {
        self.noise = builder.into().build();
    }

    pub fn regenerate_detail_noise(&mut self, builder: impl Into<NoiseBuilder>) {
        self.detail_noise = builder.into().build();
    }

    pub fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    pub fn sample_density(&self, ray_pos: Vec3) -> f32 {
        const BASE_SCALE: f32 = 1.0 / 1000.0;
        const OFFSET_SPEED: f32 = 1.0 / 100.0;

        let uvw = ray_pos * self.cloud_scale * BASE_SCALE + self.offset * OFFSET_SPEED;
        let shape = self.noise.sample_level(uvw).abs();

        let bb = self.bounding_box();
        let size = bb.size();
        let center = bb.center();
        let container_edge_fade_dst = self.edge_distance;

        let dst_from_edge_x = (ray_pos.x - bb.min.x)
            .min(bb.max.x - ray_pos.x)
            .min(container_edge_fade_dst);
        let dst_from_edge_z = (ray_pos.z - bb.min.z)
            .min(bb.max.z - ray_pos.z)
            .min(container_edge_fade_dst);
        let edge_weight = (dst_from_edge_x.min(dst_from_edge_z)) / container_edge_fade_dst;

        let weather_uv = (size.xz() * 0.5 + (ray_pos.xz() - center.xz())) / size.x.max(size.z);
        let weather_map = self.weather_map.sample_level(Vec3::new(weather_uv.x, 0.0, weather_uv.y)).x * 0.5;
        // println!("{:?}", weather_map);
        let g_min = weather_map.remap(0.0, 1.0, 0.1, 0.5);
        let g_max = weather_map.remap(0.0, 1.0, g_min, 0.9);

        // let g_max = 0.7;
        // let g_min = 0.3;

        let height_percent = (ray_pos.y - bb.min.y) / size.y;
        let height_gradient = height_percent.remap(0.0, g_min, 0.0, 1.0).clamp(0.0, 1.0)
            * height_percent.remap(1.0, g_max, 0.0, 1.0).clamp(0.0, 1.0);
        let height_gradient = height_gradient * edge_weight * self.height_map_factor;

        let normalized_shape_weights =
            self.shape_noise_weights / self.shape_noise_weights.dot(Vec4::ONE);
        let shape_fbm = shape.dot(normalized_shape_weights) * height_gradient;

        let base_shape_density = shape_fbm + self.density_offset * 0.1;

        if base_shape_density > 0.0 {
            let detail_sample_pos = uvw * self.detail_noise_scale
                + self.detail_offset * OFFSET_SPEED
                + self.offset * OFFSET_SPEED;
            let detail_noise = self.detail_noise.sample_level(detail_sample_pos).abs();

            let normalized_detail_weights =
                self.detail_weights / self.detail_weights.dot(Vec4::ONE);
            let detail_fbm = detail_noise.dot(normalized_detail_weights);
            // let detail_fbm = 0.5;

            let one_minus_shape = 1.0 - shape_fbm;
            let detail_erode_weight = one_minus_shape.powf(3.0_f32);
            let cloud_density = base_shape_density
                - (1.0 - detail_fbm) * detail_erode_weight * self.detail_noise_weight;
            // println!("{:?}", cloud_density * self.density_multiplier);
            return cloud_density * self.density_multiplier;
        }

        0.0
    }

    pub fn light_march(&self, mut p: Vec3, world_space_light_pos0: Vec3) -> f32 {
        let dir_to_light = world_space_light_pos0;
        let dst_inside_box = self.bounding_box().dst(p, dir_to_light).y;
        let step_size = dst_inside_box / self.num_steps_light as f32;
        p += dir_to_light * step_size;

        let mut total_density = 0.0;
        let step_size_f32 = step_size;

        for _ in 0..self.num_steps_light {
            let density = self.sample_density(p);
            total_density += density.max(0.0);
            p += dir_to_light * step_size_f32;
        }

        let transmittance = beer(total_density * self.light_absorption_toward_sun * step_size_f32);
        transmittance.lerp(1.0, self.darkness_threshold)
    }
}

impl Visitable for Cloud {
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_cloud(self);
    }
}

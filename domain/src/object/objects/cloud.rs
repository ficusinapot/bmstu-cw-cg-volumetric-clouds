use cgmath::num_traits::clamp;
use egui::Color32;
use glam::{IVec3, Vec3, Vec4};

use crate::object::objects::BoundingBox;
use crate::object::objects::texture3d::{Worley, WorleyBuilder};
use crate::visitor::{Visitable, Visitor};

fn remap(v: f32, min_old: f32, max_old: f32, min_new: f32, max_new: f32) -> f32 {
    min_new + (v - min_old) * (max_new - min_new) / (max_old - min_old)
}

fn square_uv(uv: (f32, f32), screen_params: (f32, f32)) -> (f32, f32) {
    let width = screen_params.0;
    let height = screen_params.1;
    let scale = 1000.0;
    let x = uv.0 * width;
    let y = uv.1 * height;
    (x / scale, y / scale)
}

fn hg(a: f32, g: f32) -> f32 {
    let g2 = g * g;
    (1.0 - g2) / (4.0 * std::f32::consts::PI * (1.0 + g2 - 2.0 * g * a).powf(1.5))
}

fn phase(a: f32, phase_params: (f32, f32, f32, f32)) -> f32 {
    let blend = 0.5;
    let hg_blend = hg(a, phase_params.0) * (1.0 - blend) + hg(a, -phase_params.1) * blend;
    phase_params.2 + hg_blend * phase_params.3
}

fn beer(d: f32) -> f32 {
    (-d).exp()
}

fn remap01(v: f32, low: f32, high: f32) -> f32 {
    (v - low) / (high - low)
}


pub struct Cloud {
    bounding_box: BoundingBox,
    noise: Worley,
    detail_noise: Worley,
    pub clouds_offset: Vec3,
    pub cloud_scale: f32,
    pub density_threshold: f32,
    pub density_offset: f32,
    pub density_multiplier: f32,

    pub num_steps: usize,
    pub ray_offset_strength: f32,

    pub alpha_threshold: u8,
    pub color: Color32,

    pub params: Vec4,
    pub map_size: IVec3,
    pub detail_noise_scale: f32,
    pub detail_noise_weight: f32,
    pub detail_weights: Vec3,
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
}

impl Cloud {
    pub fn new(bounding_box: impl Into<BoundingBox>) -> Self {
        Self {
            bounding_box: bounding_box.into(),
            noise: Worley::default(),
            detail_noise: Default::default(),
            clouds_offset: Default::default(),
            cloud_scale: 0.0,
            density_threshold: 0.0,
            density_multiplier: 0.0,
            density_offset: 0.0,
            num_steps: 0,
            ray_offset_strength: 0.0,
            alpha_threshold: 0,
            color: Default::default(),
            params: Default::default(),
            map_size: Default::default(),
            detail_noise_scale: 0.0,
            detail_noise_weight: 0.0,
            detail_weights: Default::default(),
            shape_noise_weights: Default::default(),
            phase_params: Default::default(),
            shape_offset: Default::default(),
            detail_offset: Default::default(),
            light_absorption_toward_sun: 0.0,
            light_absorption_through_cloud: 0.0,
            darkness_threshold: 0.0,
            light_color: Default::default(),
            col_a: Default::default(),
            col_b: Default::default(),
        }
    }

    pub fn regenerate_noise(&mut self, worley_builder: WorleyBuilder) {
        self.noise = Worley::build(worley_builder);
    }

    pub fn regenerate_detail_noise(&mut self, worley_builder: WorleyBuilder) {
        self.detail_noise = Worley::build(worley_builder);
    }

    pub fn with_noise(mut self, worley_builder: WorleyBuilder) -> Self {
        self.noise = Worley::build(worley_builder);
        self
    }

    pub fn with_detail_noise(mut self, worley_builder: WorleyBuilder) -> Self {
        self.detail_noise = Worley::build(worley_builder);
        self
    }

    pub fn with_clouds_offset(mut self, clouds_offset: Vec3) -> Self {
        self.clouds_offset = clouds_offset;
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

    pub fn with_detail_weights(mut self, detail_weights: Vec3) -> Self {
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

    pub fn with_light_absorption_through_cloud(mut self, light_absorption_through_cloud: f32) -> Self {
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

    pub fn bounding_box(&self) -> &BoundingBox {
        &self.bounding_box
    }

    pub fn sample_density(&self, ray_pos: Vec3) -> f32 {
        const BASE_SCALE: f32 = 1.0/1000.0;
        const OFFSET_SPEED:f32 = 1.0/100.0;

        let bb = self.bounding_box();
        let size = self.bounding_box().size();
        let bounds_centre = (self.bounding_box().min+self.bounding_box().max) * 0.5;
        let uvw = (size * 0.5 + ray_pos) * BASE_SCALE * self.cloud_scale;
        let shape_sample_pos = uvw + self.shape_offset * OFFSET_SPEED /* * time thingy here */;


        const CONTAINER_EDGE_FADE_DST: f32 = 50.0;
        let dst_from_edge_x = CONTAINER_EDGE_FADE_DST.min((ray_pos.x - bb.min.x).min(bb.max.x - ray_pos.x));
        let dst_from_edge_z = CONTAINER_EDGE_FADE_DST.min((ray_pos.z - bb.min.z).min(bb.max.z - ray_pos.z));
        let edge_weight = dst_from_edge_z.min(dst_from_edge_x) / CONTAINER_EDGE_FADE_DST;

        let g_min = 0.2;
        let g_max = 0.7;
        let height_percent = (ray_pos.y - bb.min.y) / size.y;
        let height_gradient = clamp(remap(height_percent, 0.0, g_min, 0.0, 1.0), 0.0, 1.0)
            * clamp(remap(height_percent, 1.0, g_max, 0.0, 1.0), 0.0, 1.0);
        let height_gradient = height_gradient * edge_weight;
        
        let shape_noise = self.noise.get(uvw);
        let normalized_shape_weights = self.shape_noise_weights / self.shape_noise_weights.dot(Vec4::splat(1.0));
        let shape_fbm = shape_noise.dot(normalized_shape_weights) * height_gradient;
        let base_shape_density = shape_fbm + self.density_offset * 0.1;

        if base_shape_density > 0.0 {
            // Sample detail noise
            let detail_sample_pos = uvw * self.detail_noise_scale + self.detail_offset * OFFSET_SPEED;
            
            let detail_noise = self.detail_noise.get(detail_sample_pos);

            let normalized_detail_weights = self.detail_weights / self.detail_weights.dot(Vec3::ONE);
            let detail_fbm = detail_noise.dot(normalized_detail_weights.extend(0.0));

            // Subtract detail noise from base shape (weighted by inverse density for edge erosion)
            let one_minus_shape = 1.0 - shape_fbm;
            let detail_erode_weight = one_minus_shape * one_minus_shape * one_minus_shape;
            let cloud_density = base_shape_density - (1.0 - detail_fbm) * detail_erode_weight * self.detail_noise_weight;

            cloud_density * self.density_multiplier * 0.1
        } else {
            0.0
        }
    }

    pub fn light_march(&self, position: Vec3) -> f32 {
        0.0
    }
}

impl Visitable for Cloud {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_cloud(self);
    }
}

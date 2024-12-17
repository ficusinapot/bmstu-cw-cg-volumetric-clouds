use egui::Color32;
use glam::{Vec3, Vec4, Vec4Swizzles};

use domain::object::camera::Camera;
use domain::object::objects::{Cloud, Sun};
use domain::object::objects::cloud::{beer, hg, phase, CloudBuilder};
use domain::object::objects::texture3d::{PerlinBuilder, WorleyBuilder};
use domain::visitor::{Visitable, Visitor};

pub struct DrawVisitorTest<'a> {
    camera: &'a Camera,
    sun: &'a Sun,
}

impl<'a> DrawVisitorTest<'a> {
    pub fn new(camera: &'a Camera, sun: &'a Sun) -> Self {
        Self {
            camera,
            sun
        }
    }
}

impl<'a> Visitor for DrawVisitorTest<'a> {

    fn visit_cloud(&mut self, cloud: &Cloud) {
        use rayon::prelude::*;

        let bb = cloud.bounding_box();
        let (w, h) = (1056, 900);
        
        let sun_pos = self.sun.get_pos();
        let light_color: Vec4 = cloud
            .light_color
            .to_array()
            .map(|x| x as f32 / 255.0)
            .into();

        let mut img = egui::ColorImage::new([w, h], Color32::TRANSPARENT);
        let ray_origin = self.camera.pos();
        img.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let i = idx / w;
                let j = idx % w;

                let ray_dir = (self.camera.egui_to_world(i, j, 1056, 900) - ray_origin).normalize();

                let ray_box_info = bb.dst(ray_origin, ray_dir);
                let dst_to_box = ray_box_info.x;
                let dst_inside_box = ray_box_info.y;

                if dst_inside_box <= 0.0 {
                    *pixel = Color32::TRANSPARENT
                } else {
                    let mut dst_travelled = 0.0;
                    let dst_limit = dst_inside_box;
                    let step_size = dst_inside_box / cloud.num_steps as f32;
                    let mut transmittance = 1.0;
                    let mut light_energy = 0.0;

                    let entry_point = ray_origin + dst_to_box * ray_dir;
                    let cos_angle = ray_dir.dot(sun_pos);
                    let phase = phase(cos_angle.abs(), cloud.phase_params);

                    while dst_travelled < dst_limit {
                        let ray_pos = entry_point + ray_dir * dst_travelled;
                        let density = cloud.sample_density(ray_pos);
                        if density > 0.1 {
                            let light_transmittance = cloud.light_march(ray_pos, sun_pos);
                            light_energy +=
                                density * step_size * transmittance * light_transmittance * phase;
                            transmittance *=
                                beer(density * step_size * cloud.light_absorption_through_cloud);
                            if transmittance < 0.01 {
                                break;
                            }
                        }
                        dst_travelled += step_size;
                    }

                    *pixel = if transmittance >= 0.01 {
                        Color32::TRANSPARENT
                    } else {
                        let focused_eye_cos = cos_angle
                            .clamp(-std::f32::consts::PI, std::f32::consts::PI)
                            .powf(cloud.params.x);
                        let sun = hg(focused_eye_cos, 0.995).clamp(-1.0, 1.0) * transmittance;

                        let cloud_col = light_energy * light_color.xyz();
                        let col = cloud_col.clamp(Vec3::ZERO, Vec3::ONE) * (1.0 - sun)
                            + light_color.xyz() * sun;
                        let (r, g, b) = col.into();
                        Color32::from_rgba_unmultiplied(
                            (r * 255.0) as u8,
                            (g * 255.0) as u8,
                            (b * 255.0) as u8,
                            255,
                        )
                    };
                }
            });
    }
}


pub struct DrawVisitorTest2<'a> {
    camera: &'a Camera,
    sun: &'a Sun,
}


impl<'a> DrawVisitorTest2<'a> {
    pub fn new(camera: &'a Camera, sun: &'a Sun) -> Self {
        Self {
            camera,
            sun
        }
    }
}

impl<'a> Visitor for DrawVisitorTest2<'a> {

    fn visit_cloud(&mut self, cloud: &Cloud) {
        use rayon::prelude::*;

        let bb = cloud.bounding_box();
        let (w, h) = (1056, 900);

        let sun_pos = self.sun.get_pos();
        let light_color: Vec4 = cloud
            .light_color
            .to_array()
            .map(|x| x as f32 / 255.0)
            .into();

        let mut img = egui::ColorImage::new([w, h], Color32::TRANSPARENT);
        let ray_origin = self.camera.pos();
        img.pixels
            .iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let i = idx / w;
                let j = idx % w;

                let ray_dir = (self.camera.egui_to_world(i, j, 1056, 900) - ray_origin).normalize();

                let ray_box_info = bb.dst(ray_origin, ray_dir);
                let dst_to_box = ray_box_info.x;
                let dst_inside_box = ray_box_info.y;

                if dst_inside_box <= 0.0 {
                    *pixel = Color32::TRANSPARENT
                } else {
                    let mut dst_travelled = 0.0;
                    let dst_limit = dst_inside_box;
                    let step_size = dst_inside_box / cloud.num_steps as f32;
                    let mut transmittance = 1.0;
                    let mut light_energy = 0.0;

                    let entry_point = ray_origin + dst_to_box * ray_dir;
                    let cos_angle = ray_dir.dot(sun_pos);
                    let phase = phase(cos_angle.abs(), cloud.phase_params);

                    while dst_travelled < dst_limit {
                        let ray_pos = entry_point + ray_dir * dst_travelled;
                        let density = cloud.sample_density(ray_pos);
                        if density > 0.1 {
                            let light_transmittance = cloud.light_march(ray_pos, sun_pos);
                            light_energy +=
                                density * step_size * transmittance * light_transmittance * phase;
                            transmittance *=
                                beer(density * step_size * cloud.light_absorption_through_cloud);
                            if transmittance < 0.01 {
                                break;
                            }
                        }
                        dst_travelled += step_size;
                    }

                    *pixel = if transmittance >= 0.01 {
                        Color32::TRANSPARENT
                    } else {
                        let focused_eye_cos = cos_angle
                            .clamp(-std::f32::consts::PI, std::f32::consts::PI)
                            .powf(cloud.params.x);
                        let sun = hg(focused_eye_cos, 0.995).clamp(-1.0, 1.0) * transmittance;

                        let cloud_col = light_energy * light_color.xyz();
                        let col = cloud_col.clamp(Vec3::ZERO, Vec3::ONE) * (1.0 - sun)
                            + light_color.xyz() * sun;
                        let (r, g, b) = col.into();
                        Color32::from_rgba_unmultiplied(
                            (r * 255.0) as u8,
                            (g * 255.0) as u8,
                            (b * 255.0) as u8,
                            255,
                        )
                    };
                }
            });
    }
}

fn main() {
    let noise = WorleyBuilder::new()
        .with_seed(0)
        .with_num_points_a(6)
        .with_num_points_b(12)
        .with_num_points_c(22)
        .with_tile(1.0)
        .with_resolution(128)
        .with_color_mask(Vec4::new(0.9, 1.0, 1.0, 1.0))
        .with_persistence(0.84)
        .with_invert_noise(true);

    let detail_noise = WorleyBuilder::new()
        .with_seed(0)
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
        .with_bounding_box((Vec3::new(-3.5, 1.9, -3.5), Vec3::new(3.5, 2.5, 3.5)))
        .with_shape_offset(Vec3::ZERO)
        .with_detail_offset(Vec3::ZERO)
        .with_cloud_scale(200.0)
        .with_density_threshold(0.95)
        .with_density_multiplier(360.0)
        .with_num_steps(130)
        .with_num_steps_light(10)
        .with_density_offset(-9.30)
        .with_noise(noise)
        .with_shape_noise_weights(Vec4::new(3.0, 6.0, 5.0, 1.0))
        .with_detail_noise(detail_noise)
        .with_detail_noise_weight(1.0)
        .with_detail_weights(Vec4::new(4.0, 1.5, 1.5, 3.0))
        .with_detail_noise_scale(1.09)
        .with_light_absorption_through_cloud(1.8)
        .with_light_absorption_toward_sun(0.55)
        .with_phase_params(Vec4::new(0.00, 0.48, 0.37, 0.99))
        .with_darkness_threshold(0.18)
        .with_edge_distance(1.0)
        .with_ray_offset_strength(0.0)
        .with_volume_offset(0.0)
        .with_height_map_factor(2.0)
        .with_clouds_offset(Vec3::new(0.0, 0.0, 0.0))
        .with_weather_noise(
            PerlinBuilder::new()
                .with_num_points_a(1)
                .with_num_points_b(27)
                .with_num_points_c(29)
                .with_tile(1.0)
                .with_resolution(128)
                .with_color_mask(Vec4::new(1.0, 1.0, 1.0, 1.0))
                .with_persistence(0.8)
                .with_invert_noise(false),
        );
    
    let camera = Camera::default();

    let sun = Sun::new(10.0, -135.0);
    let mut visitor = DrawVisitorTest::new(&camera, &sun);
    let mut visitor2 = DrawVisitorTest2::new(&camera, &sun);
    
    let mut cloud = cloud_params.build();

    use std::time::Instant;

    for i in (0..=300).step_by(20) {
        let start_time = Instant::now();
        cloud.num_steps = i;
        let nums = 200;
        for _ in 0..nums {
            visitor.visit_cloud(&cloud);
        }
        let elapsed_time = start_time.elapsed() / nums;
        println!("1: steps: {}, time: {:?}", i, elapsed_time);

        let start_time = Instant::now();
        for _ in 0..nums {
            visitor2.visit_cloud(&cloud);
        }
        let elapsed_time = start_time.elapsed() / nums;
        println!("2: steps: {}, time: {:?}", i, elapsed_time);
        
    }
    
}
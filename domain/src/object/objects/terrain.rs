#![allow(unused)]

use egui::Color32;
use glam::Vec3;
use rayon::iter::IntoParallelIterator;
use std::ops::{Deref, DerefMut};

use crate::object::objects::texture3d::{Perlin, PerlinBuilder};
use crate::object::objects::textures::texture3d::{Worley, WorleyBuilder};
use crate::object::objects::BoundingBox;
use crate::visitor::{Visitable, Visitor};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct TerrainBuilder {
    pub bounding_box: BoundingBox,
    pub scale: usize,
    pub noise_weight: glam::Vec4,
    pub noise: PerlinBuilder,
    pub top_color: Color32,
    pub bottom_color: Color32,
    pub shadow_threshold: f32,
    pub num_shadows_steps: usize,
    pub density_scale: f32,
    pub diffuse_factor: f32,
}

impl TerrainBuilder {
    pub fn build(self) -> Terrain {
        Terrain::build(self)
    }

    pub fn with_bounding_box(mut self, bounding_box: impl Into<BoundingBox>) -> Self {
        self.bounding_box = bounding_box.into();
        self
    }

    pub fn with_scale(mut self, scale: usize) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_noise(mut self, noise_builder: PerlinBuilder) -> Self {
        self.noise = noise_builder;
        self
    }

    pub fn with_top_color(mut self, top_color: Color32) -> Self {
        self.top_color = top_color;
        self
    }

    pub fn with_bottom_color(mut self, bottom_color: Color32) -> Self {
        self.bottom_color = bottom_color;
        self
    }

    pub fn with_shadow_threshold(mut self, shadow_threshold: f32) -> Self {
        self.shadow_threshold = shadow_threshold;
        self
    }

    pub fn with_density_scale(mut self, density_scale: f32) -> Self {
        self.density_scale = density_scale;
        self
    }

    pub fn with_diffuse_factor(mut self, diffuse_factor: f32) -> Self {
        self.diffuse_factor = diffuse_factor;
        self
    }

    pub fn with_num_shadows_steps(mut self, num_shadows_steps: usize) -> Self {
        self.num_shadows_steps = num_shadows_steps;
        self
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TriangleMesh(pub Vec3, pub Vec3, pub Vec3);
impl TriangleMesh {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Self(v0, v1, v2)
    }
    pub fn normal(&self) -> Vec3 {
        let edge1 = self.1 - self.0;
        let edge2 = self.2 - self.0;
        edge1.cross(edge2).normalize()
    }

    pub fn center(&self) -> Vec3 {
        (self.0 + self.1 + self.2) / 3.0
    }

    pub fn max(&self) -> Vec3 {
        (self.0.max(self.1).max(self.2))
    }

    pub fn to_tuple(&self) -> (Vec3, Vec3, Vec3) {
        (self.0, self.1, self.2)
    }

    pub fn to_array(&self) -> [Vec3; 3] {
        [self.0, self.1, self.2]
    }

    pub fn append(&self, v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Self(v0 + self.0, v1 + self.1, v2 + self.2)
    }

    pub fn contains(&self, point: Vec3) -> bool {
        let (v0, v1, v2) = (self.0, self.1, self.2);

        let edge0 = v1 - v0;
        let edge1 = v2 - v1;
        let edge2 = v0 - v2;

        let c0 = (point - v0).cross(edge0);
        let c1 = (point - v1).cross(edge1);
        let c2 = (point - v2).cross(edge2);

        let has_same_sign = |a: &Vec3, b: &Vec3| a.dot(*b) >= 0.0;

        has_same_sign(&c0, &c1) && has_same_sign(&c1, &c2) && has_same_sign(&c2, &c0)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Terrain {
    pub terrain_builder: TerrainBuilder,
    pub triangles: Vec<(TriangleMesh, (Vec3, Vec3, Vec3))>,
    perlin: Perlin,
}

impl Deref for Terrain {
    type Target = TerrainBuilder;

    fn deref(&self) -> &Self::Target {
        &self.terrain_builder
    }
}

impl DerefMut for Terrain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terrain_builder
    }
}

impl Terrain {
    pub fn build(terrain_builder: TerrainBuilder) -> Self {
        let bb = terrain_builder.bounding_box;
        let min = bb.min;
        let max = bb.max;
        let scl = terrain_builder.scale;
        let perlin = terrain_builder.noise.build();

        let mut res = Self {
            terrain_builder,
            triangles: vec![],
            perlin,
        };

        res.generate_grid();
        res
    }

    pub fn regenerate_noise(&mut self, worley_builder: PerlinBuilder) {
        self.perlin = worley_builder.build();
    }

    pub fn sample_height(&self, x: f32, z: f32) -> Vec3 {
        let bb = self.bounding_box;
        let sample_pos = bb.min + Vec3::new(x, self.noise_weight.x, z) / bb.size();

        let worley_height = self.perlin.sample_level(sample_pos).x;
        Vec3::new(
            0.0,
            self.bounding_box.min.y
                + worley_height * (self.bounding_box.max.y - self.bounding_box.min.y),
            0.0,
        ) * self.noise_weight.y
    }

    pub fn generate_grid(&mut self) {
        use rayon::prelude::*;
        let bb = self.bounding_box;
        let min = bb.min;
        let max = bb.max;
        let scale = self.scale;

        let sample_y = self.noise_weight.x;
        let sample_z = self.noise_weight.y;
        let noise = &self.perlin;

        let vec: Vec<_> = (0..self.scale)
            .into_par_iter()
            .flat_map(|z| {
                (0..=self.scale)
                    .into_par_iter()
                    .map(move |x| {
                        let x_frac = x as f32 / scale as f32;
                        let z_frac = z as f32 / scale as f32;
                        let next_z_frac = (z + 1) as f32 / scale as f32;

                        let base_x = min.x + (max.x - min.x) * x_frac;
                        let base_z = min.z + (max.z - min.z) * z_frac;
                        let next_z = min.z + (max.z - min.z) * next_z_frac;

                        let sample_pos = bb.min
                            + Vec3::new(base_x, sample_y, base_z)
                                / Vec3::new(bb.size().x, 1.0, bb.size().z);
                        let sample_pos2 = bb.min
                            + Vec3::new(base_x, sample_y, next_z)
                                / Vec3::new(bb.size().x, 1.0, bb.size().z);

                        // let sample_pos = bb.min + Vec3::new(base_x, sample_y, base_z) / bb.size();
                        let worley_height =
                            bb.min.y + noise.sample_level(sample_pos).x * (bb.max.y - bb.min.y);
                        // println!("{:?}", worley_height);

                        // let sample_pos2 = bb.min + Vec3::new(base_x, sample_y, next_z) / bb.size();
                        let worley_height2 =
                            bb.min.y + noise.sample_level(sample_pos2).x * (bb.max.y - bb.min.y);

                        let vec = Vec3::new(base_x, worley_height, base_z);
                        let vec2 = Vec3::new(base_x, worley_height2, next_z);

                        vec![vec, vec2]
                    })
                    .flatten()
            })
            .collect();

        let res: Vec<_> = vec
            .par_windows(3)
            .enumerate()
            .filter_map(|(i, v)| {
                let v0 = v[2];
                let v1 = v[1];
                let v2 = v[0];

                if i == self.scale * 2 - 1
                    || i == self.scale * 2 - 2
                    || i >= 2 * self.scale
                        && ((i - 2 * (i / (2 * self.scale) - 1)) % (self.scale * 2) != 0
                            && ((i - 1) - 2 * (i / (2 * self.scale) - 1)) % (self.scale * 2) != 0)
                {
                    Some(TriangleMesh::new(
                        if i % 2 == 0 { v2 } else { v0 },
                        v1,
                        if i % 2 == 0 { v0 } else { v2 },
                    ))
                } else {
                    None
                }
            })
            .collect();

        self.triangles = res
            .par_iter()
            .map(|i| {
                let normals =
                    res.iter()
                        .fold((Vec3::ZERO, Vec3::ZERO, Vec3::ZERO), |mut acc, j| {
                            let arr = j.to_array();
                            if arr.contains(&i.0) {
                                acc.0 += j.normal();
                            }
                            if arr.contains(&i.1) {
                                acc.1 += j.normal();
                            }
                            if arr.contains(&i.2) {
                                acc.2 += j.normal();
                            }
                            acc
                        });
                (
                    i.clone(),
                    (normals.0 / 6.0, normals.1 / 6.0, normals.2 / 6.0),
                )
            })
            .collect();
    }
}

impl Visitable for Terrain {
    fn accept(&self, visitor: &mut impl Visitor) {
        visitor.visit_terrain(self)
    }
}

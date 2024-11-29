#![allow(unused)]

use std::ops::{Deref, DerefMut};

use crate::object::objects::cloud::CloudBuilder;
use crate::object::objects::textures::{Worley, WorleyBuilder};
use crate::object::objects::{BoundingBox, Cloud};
use crate::visitor::{Visitable, Visitor};
use glam::Vec3;
use rand::prelude::StdRng;
use rand::{random, Rng, SeedableRng};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct TerrainBuilder {
    pub bounding_box: BoundingBox,
    pub scale: f32,
    pub noise: WorleyBuilder,
}

impl TerrainBuilder {
    pub fn build(self) -> Terrain {
        Terrain::build(self)
    }

    pub fn with_bounding_box(mut self, bounding_box: impl Into<BoundingBox>) -> Self {
        self.bounding_box = bounding_box.into();
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_noise(mut self, noise_builder: WorleyBuilder) -> Self {
        self.noise = noise_builder;
        self
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Terrain {
    terrain_builder: TerrainBuilder,
    pub triangle_strip: Vec<Vec3>,
    worley: Worley,
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
        use rayon::prelude::*;
        let bb = terrain_builder.bounding_box;
        let min = bb.min;
        let max = bb.max;
        let scl = terrain_builder.scale as usize;
        let worley = terrain_builder.noise.build();

        let triangle_strip: Vec<Vec3> = (0..scl)
            .into_par_iter()
            .flat_map(|z| {
                (0..=scl)
                    .into_par_iter()
                    .map(move |x| {
                        let x_frac = x as f32 / scl as f32;
                        let z_frac = z as f32 / scl as f32;
                        let next_z_frac = (z + 1) as f32 / scl as f32;

                        let base_x = min.x + (max.x - min.x) * x_frac;
                        let base_z = min.z + (max.z - min.z) * z_frac;
                        let next_z = min.z + (max.z - min.z) * next_z_frac;

                        let vec = Vec3::new(base_x, min.y, base_z);
                        let vec2 = Vec3::new(base_x, min.y, next_z);
                        vec![
                            vec,
                            Vec3::new(vec.x, vec2.y, vec.z + (max.z - min.z) / scl as f32),
                        ]
                    })
                    .flatten()
            })
            .collect();
        Self {
            terrain_builder,
            triangle_strip,
            worley,
        }
    }

    pub fn at(&self, x: f32, z: f32) -> f32 {
        let worley_height = self.worley.sample_level(Vec3::new(x, 0.5, z)).z;
        self.bounding_box.min.y
            + worley_height * (self.bounding_box.max.y - self.bounding_box.min.y)
    }
}

impl Visitable for Terrain {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_terrain(self)
    }
}

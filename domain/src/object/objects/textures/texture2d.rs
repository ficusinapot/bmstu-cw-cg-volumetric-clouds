use std::ops::{Deref, DerefMut, Index, IndexMut};

use glam::{UVec2, Vec2, Vec4};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Default, Clone, Debug, PartialEq)]
struct RWTexture2D<T> {
    data: Vec<T>,
    x: usize,
    y: usize,
}

impl<T> Index<UVec2> for RWTexture2D<T> {
    type Output = T;

    fn index(&self, coords: UVec2) -> &Self::Output {
        &self.data[coords.y as usize * self.x + coords.x as usize]
    }
}

impl<T: Copy> RWTexture2D<T> {
    fn sample_level(&self, uv: Vec2, _level: usize) -> T {
        assert_ne!(self.x, 0);
        assert_ne!(self.y, 0);
        let u = ((uv.x * self.x as f32) as isize).rem_euclid(self.x as isize) as u32;
        let v = ((uv.y * self.y as f32) as isize).rem_euclid(self.y as isize) as u32;

        self[(u, v).into()]
    }
}

impl<T> IndexMut<UVec2> for RWTexture2D<T> {
    fn index_mut(&mut self, coords: UVec2) -> &mut Self::Output {
        &mut self.data[coords.y as usize * self.x + coords.x as usize]
    }
}
//
// #[allow(unused)]
// #[derive(Default, Clone, Copy, Debug)]
// pub struct PerlinNoiseBuilder {
//     pub seed: u64,
// }
//
// impl PerlinNoiseBuilder {
//     pub(crate) fn build(self) -> PerlinNoise {
//         PerlinNoise::build(self)
//     }
// }
//
// #[derive(Default, Debug)]
// pub struct PerlinNoise {
//     texture2d: RWTexture2D<Vec4>,
//     pub builder: PerlinNoiseBuilder,
// }
//
// impl Deref for PerlinNoise {
//     type Target = PerlinNoiseBuilder;
//     fn deref(&self) -> &Self::Target {
//         &self.builder
//     }
// }
//
// impl DerefMut for PerlinNoise {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.builder
//     }
// }
//
// impl PerlinNoise {
//     pub fn build(blue_noise_builder: PerlinNoiseBuilder) -> Self {
//         let _rng = StdRng::seed_from_u64(blue_noise_builder.seed);
//
//         todo!()
//     }
//     fn generate_noise(&mut self) {
//         todo!()
//     }
//
//     pub fn sample_level(&self, vec3: Vec3) -> Vec4 {
//         self.texture2d.sample_level(vec3, 0)
//     }
// }

use std::ops::{Deref, DerefMut, Index, IndexMut};

use glam::{IVec3, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

const OFFSETS: [IVec3; 27] = [
    // centre
    IVec3::new(0, 0, 0),
    // front face
    IVec3::new(0, 0, 1),
    IVec3::new(-1, 1, 1),
    IVec3::new(-1, 0, 1),
    IVec3::new(-1, -1, 1),
    IVec3::new(0, 1, 1),
    IVec3::new(0, -1, 1),
    IVec3::new(1, 1, 1),
    IVec3::new(1, 0, 1),
    IVec3::new(1, -1, 1),
    // back face
    IVec3::new(0, 0, -1),
    IVec3::new(-1, 1, -1),
    IVec3::new(-1, 0, -1),
    IVec3::new(-1, -1, -1),
    IVec3::new(0, 1, -1),
    IVec3::new(0, -1, -1),
    IVec3::new(1, 1, -1),
    IVec3::new(1, 0, -1),
    IVec3::new(1, -1, -1),
    // ring around centre
    IVec3::new(-1, 1, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(-1, -1, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(1, 1, 0),
    IVec3::new(1, 0, 0),
    IVec3::new(1, -1, 0),
];

#[derive(Default, Clone, Debug, PartialEq)]
struct Texture3D<T> {
    data: Vec<T>,
    x: usize,
    y: usize,
    z: usize,
}

impl<T> Index<UVec3> for Texture3D<T> {
    type Output = T;

    fn index(&self, coords: UVec3) -> &Self::Output {
        &self.data
            [coords.z as usize * self.x * self.y + coords.y as usize * self.x + coords.x as usize]
    }
}

impl<T: Copy> Texture3D<T> {
    fn sample_level(&self, uvw: Vec3, _level: usize) -> T {
        assert_ne!(self.x, 0);
        assert_ne!(self.y, 0);
        assert_ne!(self.z, 0);
        let u = ((uvw.x * self.x as f32) as isize).rem_euclid(self.x as isize) as u32;
        let v = ((uvw.y * self.y as f32) as isize).rem_euclid(self.y as isize) as u32;
        let w = ((uvw.z * self.z as f32) as isize).rem_euclid(self.z as isize) as u32;

        self[(u, v, w).into()]
    }
}

impl<T> IndexMut<UVec3> for Texture3D<T> {
    fn index_mut(&mut self, coords: UVec3) -> &mut Self::Output {
        &mut self.data
            [coords.z as usize * self.x * self.y + coords.y as usize * self.x + coords.x as usize]
    }
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct WorleyBuilder {
    pub seed: u64,
    pub num_points_a: usize,
    pub num_points_b: usize,
    pub num_points_c: usize,
    pub persistence: f32,
    pub invert_noise: bool,
    pub resolution: usize,
    pub tile: f32,
    pub color_mask: Vec4,
}

impl WorleyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_num_points_a(mut self, num_points_a: usize) -> Self {
        self.num_points_a = num_points_a;
        self
    }

    pub fn with_num_points_b(mut self, num_points_b: usize) -> Self {
        self.num_points_b = num_points_b;
        self
    }

    pub fn with_num_points_c(mut self, num_points_c: usize) -> Self {
        self.num_points_c = num_points_c;
        self
    }

    pub fn with_persistence(mut self, persistence: f32) -> Self {
        self.persistence = persistence;
        self
    }

    pub fn with_invert_noise(mut self, invert_noise: bool) -> Self {
        self.invert_noise = invert_noise;
        self
    }

    pub fn with_resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn with_tile(mut self, tile: f32) -> Self {
        self.tile = tile;
        self
    }

    pub fn with_color_mask(mut self, color: Vec4) -> Self {
        self.color_mask = color;
        self
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Worley {
    points_a: Vec<Vec3>,
    points_b: Vec<Vec3>,
    points_c: Vec<Vec3>,
    texture3d: Texture3D<Vec4>,
    pub builder: WorleyBuilder,
}

impl Deref for Worley {
    type Target = WorleyBuilder;
    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl DerefMut for Worley {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}
impl Worley {
    fn create_worley_points_buffer(rng: &mut impl Rng, num_cells: usize) -> Vec<Vec3> {
        let mut points = vec![Vec3::ZERO; num_cells * num_cells * num_cells];
        let cell_size = 1.0 / num_cells as f32;

        for x in 0..num_cells {
            for y in 0..num_cells {
                for z in 0..num_cells {
                    points[x + num_cells * (y + z * num_cells)] =
                        Vec3::new(x as f32, y as f32, z as f32) * cell_size
                            + Vec3::new(
                                rng.gen_range(0.0..=1.0),
                                rng.gen_range(0.0..=1.0),
                                rng.gen_range(0.0..=1.0),
                            ) * cell_size;
                }
            }
        }

        points
    }

    fn worley(points: &[Vec3], num_cells: usize, sample_pos: Vec3, tile: f32) -> f32 {
        let sample_pos = (sample_pos * tile) % 1.;
        let cell_id = (sample_pos * num_cells as f32).floor().as_ivec3();
        let mut min_sqrt_dist: f32 = 1.0;

        let num_cells = num_cells as i32;
        for i in OFFSETS {
            let adj_id = cell_id + i;
            if adj_id.min_element() == -1 || adj_id.max_element() == num_cells {
                let wrapped_id = (adj_id + num_cells) % num_cells;
                let adj_cell_index =
                    wrapped_id.x + num_cells * (wrapped_id.y + wrapped_id.z * num_cells);
                let wrapped_point = points[adj_cell_index as usize];
                for j in OFFSETS {
                    let sample_offset = sample_pos - (wrapped_point + j.as_vec3());
                    min_sqrt_dist = min_sqrt_dist.min(sample_offset.dot(sample_offset));
                }
            } else {
                let adj_cell_index = adj_id.x + num_cells * (adj_id.y + adj_id.z * num_cells);
                let sample_offset = sample_pos - points[adj_cell_index as usize];
                min_sqrt_dist = min_sqrt_dist.min(sample_offset.dot(sample_offset));
            }
        }

        min_sqrt_dist.sqrt()
    }
}

// pub type Perlin = Worley;
// pub type PerlinBuilder = WorleyBuilder;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct PerlinBuilder {
    pub seed: u64,
    pub num_points_a: usize,
    pub num_points_b: usize,
    pub num_points_c: usize,
    pub persistence: f32,
    pub invert_noise: bool,
    pub resolution: usize,
    pub tile: f32,
    pub color_mask: Vec4,
}

impl PerlinBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_num_points_a(mut self, num_points_a: usize) -> Self {
        self.num_points_a = num_points_a;
        self
    }

    pub fn with_num_points_b(mut self, num_points_b: usize) -> Self {
        self.num_points_b = num_points_b;
        self
    }

    pub fn with_num_points_c(mut self, num_points_c: usize) -> Self {
        self.num_points_c = num_points_c;
        self
    }

    pub fn with_persistence(mut self, persistence: f32) -> Self {
        self.persistence = persistence;
        self
    }

    pub fn with_invert_noise(mut self, invert_noise: bool) -> Self {
        self.invert_noise = invert_noise;
        self
    }

    pub fn with_resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn with_tile(mut self, tile: f32) -> Self {
        self.tile = tile;
        self
    }

    pub fn with_color_mask(mut self, color: Vec4) -> Self {
        self.color_mask = color;
        self
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Perlin {
    texture3d: Texture3D<Vec4>,
    pub builder: PerlinBuilder,
}

impl Deref for Perlin {
    type Target = PerlinBuilder;
    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl DerefMut for Perlin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}

impl Perlin {
    #[allow(clippy::excessive_precision)]
    fn perlin(num_cells: usize, sample_pos: Vec3, tile: f32) -> f32 {
        #[inline]
        fn mod289_vec3(x: Vec3) -> Vec3 {
            x - (x / 289.0).floor() * 289.0
        }
        #[inline]
        fn mod289_vec2(x: Vec2) -> Vec2 {
            x - (x / 289.0).floor() * 289.0
        }
        #[inline]
        fn permute(x: Vec3) -> Vec3 {
            mod289_vec3((x * 34.0 + 1.0) * x)
        }
        #[inline]
        fn taylor_inv_sqrt(r: Vec3) -> Vec3 {
            Vec3::new(1.79284291400159, 1.79284291400159, 1.79284291400159)
                - r * Vec3::new(0.85373472095314, 0.85373472095314, 0.85373472095314)
        }
        #[inline]
        fn snoise(v: Vec2) -> f32 {
            let c = Vec4::new(
                0.211324865405187,  // (3.0 - sqrt(3.0)) / 6.0
                0.366025403784439,  // 0.5 * (sqrt(3.0) - 1.0)
                -0.577350269189626, // -1.0 + 2.0 * C.x
                0.024390243902439,  // 1.0 / 41.0
            );

            let i = (v + v.dot(c.yy())).floor();
            let x0 = v - i + i.dot(c.xx());

            let i1 = Vec2::new(
                if x0.x > x0.y { 1.0 } else { 0.0 },
                if x0.x > x0.y { 0.0 } else { 1.0 },
            );

            let x1 = x0 - i1 + c.xx();
            let x2 = x0 + c.zz();

            let i = mod289_vec2(i);
            let p = permute(
                permute(Vec3::new(i.y, i.y + i1.y, i.y + 1.0))
                    + Vec3::new(i.x, i.x + i1.x, i.x + 1.0),
            );

            let m = (Vec3::new(0.5, 0.5, 0.5) - Vec3::new(x0.dot(x0), x1.dot(x1), x2.dot(x2)))
                .max(Vec3::ZERO);
            let m = m * m * m * m;

            let x = 2.0 * (p * c.www()).fract() - 1.0;
            let h = x.abs() - 0.5;
            let ox = (x + 0.5).floor();
            let a0 = x - ox;

            let m = m * taylor_inv_sqrt(a0 * a0 + h * h);

            let g = Vec3::new(
                a0.x * x0.x + h.x * x0.y,
                a0.y * x1.x + h.y * x1.y,
                a0.z * x2.x + h.z * x2.y,
            );

            (130.0 * m.dot(g)) * 0.5 + 0.5
        }

        snoise(Vec2::new(
            sample_pos.z * tile * num_cells as f32,
            sample_pos.x * tile * num_cells as f32,
        ))
    }
}

#[derive(Clone, Debug)]
pub enum Noise {
    Worley(Worley),
    Perlin(Perlin),
}

impl Default for Noise {
    fn default() -> Self {
        Self::Worley(Worley::default())
    }
}

impl INoise for Noise {
    type NoiseBuilder = NoiseBuilder;
    fn sample_level(&self, vec3: Vec3) -> Vec4 {
        match self {
            Noise::Worley(x) => x.sample_level(vec3),
            Noise::Perlin(x) => x.sample_level(vec3),
        }
    }
    fn generate_noise(&mut self) {
        match self {
            Noise::Worley(x) => x.generate_noise(),
            Noise::Perlin(x) => x.generate_noise(),
        }
    }

    fn build(noise_builder: Self::NoiseBuilder) -> Self {
        noise_builder.build()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NoiseBuilder {
    WorleyBuilder(WorleyBuilder),
    PerlinBuilder(PerlinBuilder),
}

impl INoiseBuilder for NoiseBuilder {
    type Noise = Noise;
    fn build(self) -> Noise {
        match self {
            NoiseBuilder::WorleyBuilder(x) => Noise::Worley(x.build()),
            NoiseBuilder::PerlinBuilder(x) => Noise::Perlin(x.build()),
        }
    }
}

impl From<WorleyBuilder> for NoiseBuilder {
    fn from(value: WorleyBuilder) -> Self {
        Self::WorleyBuilder(value)
    }
}

impl From<PerlinBuilder> for NoiseBuilder {
    fn from(value: PerlinBuilder) -> Self {
        Self::PerlinBuilder(value)
    }
}

impl Default for NoiseBuilder {
    fn default() -> Self {
        Self::WorleyBuilder(WorleyBuilder::default())
    }
}

pub trait INoise: Sized {
    type NoiseBuilder;
    fn sample_level(&self, vec3: Vec3) -> Vec4;
    fn generate_noise(&mut self);

    fn build(noise_builder: Self::NoiseBuilder) -> Self;
}

pub trait INoiseBuilder {
    type Noise;
    fn build(self) -> Self::Noise;
}
impl INoiseBuilder for PerlinBuilder {
    type Noise = Perlin;
    fn build(self) -> Perlin {
        Perlin::build(self)
    }
}
impl INoiseBuilder for WorleyBuilder {
    type Noise = Worley;
    fn build(self) -> Worley {
        Worley::build(self)
    }
}

impl INoise for Perlin {
    type NoiseBuilder = PerlinBuilder;
    fn sample_level(&self, vec3: Vec3) -> Vec4 {
        self.texture3d.sample_level(vec3, 0)
    }
    fn generate_noise(&mut self) {
        use rayon::prelude::*;

        let params = &self.builder;
        self.texture3d
            .data
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, val)| {
                let id = UVec3::new(
                    (index % params.resolution) as u32,
                    ((index / params.resolution) % params.resolution) as u32,
                    (index / (params.resolution * params.resolution)) as u32,
                );
                let pos = id.as_vec3() / params.resolution as f32;
                let noise_sum = Perlin::perlin(params.num_points_a, pos, params.tile)
                    + Perlin::perlin(params.num_points_b, pos, params.tile) * params.persistence
                    + Perlin::perlin(params.num_points_c, pos, params.tile)
                        * params.persistence
                        * params.persistence;

                let max_val =
                    1.0 + (params.persistence) + (params.persistence * params.persistence);

                let noise_sum = noise_sum / max_val;
                let noise_sum = if params.invert_noise {
                    1.0 - noise_sum
                } else {
                    noise_sum
                };
                *val = *val * (1.0 - params.color_mask) + noise_sum * params.color_mask;
            });
    }
    fn build(noise_builder: Self::NoiseBuilder) -> Self {
        let resolution = noise_builder.resolution;
        // let mut rng = StdRng::seed_from_u64(perlin_noise_builder.seed);
        let mut w = Self {
            texture3d: Texture3D {
                data: {
                    let len = resolution * resolution * resolution;
                    vec![Vec4::ZERO; len]
                },
                x: resolution,
                y: resolution,
                z: resolution,
            },
            builder: noise_builder,
        };

        w.generate_noise();
        w
    }
}

impl INoise for Worley {
    type NoiseBuilder = WorleyBuilder;
    fn sample_level(&self, vec3: Vec3) -> Vec4 {
        self.texture3d.sample_level(vec3, 0)
    }
    fn generate_noise(&mut self) {
        use rayon::prelude::*;
        use std::sync::{Arc, Mutex};
        let min_max_lock = Arc::new(Mutex::new([i32::MAX, i32::MIN]));

        let params = &self.builder;
        self.texture3d
            .data
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, val)| {
                let id = UVec3::new(
                    (index % params.resolution) as u32,
                    ((index / params.resolution) % params.resolution) as u32,
                    (index / (params.resolution * params.resolution)) as u32,
                );
                let pos = id.as_vec3() / params.resolution as f32;

                let noise_sum =
                    Worley::worley(&self.points_a, params.num_points_a, pos, params.tile)
                        + Worley::worley(&self.points_b, params.num_points_b, pos, params.tile)
                            * params.persistence
                        + Worley::worley(&self.points_c, params.num_points_c, pos, params.tile)
                            * params.persistence
                            * params.persistence;

                let max_val =
                    1.0 + (params.persistence) + (params.persistence * params.persistence);

                let noise_sum = noise_sum / max_val;
                let noise_sum = if params.invert_noise {
                    1.0 - noise_sum
                } else {
                    noise_sum
                };
                let scaled_val = (noise_sum * 10_000_000.0) as i32;
                let mut min_max_lock = min_max_lock.lock().unwrap();
                min_max_lock[0] = min_max_lock[0].min(scaled_val);
                min_max_lock[1] = min_max_lock[1].max(scaled_val);
                drop(min_max_lock);

                *val = *val * (1.0 - params.color_mask) + noise_sum * params.color_mask;
                assert!(val.x >= 0., "{:?} {:?}", val, noise_sum);
            });

        let min_max = std::sync::Arc::into_inner(min_max_lock)
            .expect("one strong reference")
            .into_inner()
            .expect("No one holding the mutex");
        self.texture3d.data.par_iter_mut().for_each(|val| {
            let min_val = min_max[0] as f32 / 10_000_000.0;
            let max_val = min_max[1] as f32 / 10_000_000.0;
            let normalized_val = (*val - min_val) / (max_val - min_val);
            *val = *val * (1.0 - params.color_mask) + normalized_val * params.color_mask;
        });
    }

    fn build(worley_builder: Self::NoiseBuilder) -> Self {
        let resolution = worley_builder.resolution;
        let mut rng = StdRng::seed_from_u64(worley_builder.seed);
        let mut w = Self {
            points_a: Self::create_worley_points_buffer(&mut rng, worley_builder.num_points_a),
            points_b: Self::create_worley_points_buffer(&mut rng, worley_builder.num_points_b),
            points_c: Self::create_worley_points_buffer(&mut rng, worley_builder.num_points_c),
            texture3d: Texture3D {
                data: {
                    let len = resolution * resolution * resolution;
                    vec![Vec4::ZERO; len]
                },
                x: resolution,
                y: resolution,
                z: resolution,
            },
            builder: worley_builder,
        };

        w.generate_noise();
        w
    }
}

use glam::{IVec3, UVec3, Vec3, Vec4};
use log::debug;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::ops::{Index, IndexMut};

const OFFSETS: [IVec3; 27] = [
    IVec3::new(0, 0, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(-1, 1, 1),
    IVec3::new(-1, 0, 1),
    IVec3::new(-1, -1, 1),
    IVec3::new(0, 1, 1),
    IVec3::new(0, -1, 1),
    IVec3::new(1, 1, 1),
    IVec3::new(1, 0, 1),
    IVec3::new(1, -1, 1),
    IVec3::new(0, 0, -1),
    IVec3::new(-1, 1, -1),
    IVec3::new(-1, 0, -1),
    IVec3::new(-1, -1, -1),
    IVec3::new(0, 1, -1),
    IVec3::new(0, -1, -1),
    IVec3::new(1, 1, -1),
    IVec3::new(1, 0, -1),
    IVec3::new(1, -1, -1),
    IVec3::new(-1, 1, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(-1, -1, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(1, 1, 0),
    IVec3::new(1, 0, 0),
    IVec3::new(1, -1, 0),
];

struct RWTexture3D<T> {
    data: Vec<T>,
    x: usize,
    y: usize,
    z: usize,
}

impl<T> Index<UVec3> for RWTexture3D<T> {
    type Output = T;

    fn index(&self, coords: UVec3) -> &Self::Output {
        &self.data
            [coords.z as usize * self.x * self.y + coords.y as usize * self.x + coords.x as usize]
    }
}

impl<T: Copy> RWTexture3D<T> {
    fn sample_level(&self, uvw: Vec3, _level: usize) -> T {
        let u = ((uvw.x * self.x as f32) as isize).rem_euclid(self.x as isize) as u32;
        let v = ((uvw.y * self.y as f32) as isize).rem_euclid(self.y as isize) as u32;
        let w = ((uvw.z * self.z as f32) as isize).rem_euclid(self.z as isize) as u32;
        
        self[(u,v,w).into()]
    }
}

impl<T> IndexMut<UVec3> for RWTexture3D<T> {
    fn index_mut(&mut self, coords: UVec3) -> &mut Self::Output {
        &mut self.data
            [coords.z as usize * self.x * self.y + coords.y as usize * self.x + coords.x as usize]
    }
}

pub struct Worley {
    num_points_a: usize,
    num_points_b: usize,
    num_points_c: usize,
    points_a: Vec<Vec3>,
    points_b: Vec<Vec3>,
    points_c: Vec<Vec3>,
    persistence: f32,
    invert_noise: bool,
    texture3d: RWTexture3D<Vec4>,
    resolution: usize,
    tile: f32,
    color_mask: Vec4,
}

impl Worley {
    pub fn new(
        num_points_a: usize,
        num_points_b: usize,
        num_points_c: usize,
        seed: u64,
        resolution: usize,
        tile: f32,
        persistence: f32,
        invert_noise: bool,
        color_mask: Vec4,
    ) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let points_a = Self::create_worley_points_buffer(&mut rng, num_points_a);
        let points_b = Self::create_worley_points_buffer(&mut rng, num_points_b);
        let points_c = Self::create_worley_points_buffer(&mut rng, num_points_c);

        let texture3d = RWTexture3D {
            data: vec![Vec4::ZERO; resolution * resolution * resolution],
            x: resolution,
            y: resolution,
            z: resolution,
        };

        let mut worley = Self {
            texture3d,
            num_points_a,
            num_points_b,
            num_points_c,
            points_a,
            points_b,
            points_c,
            tile,
            persistence,
            invert_noise,
            resolution,
            color_mask,
        };
        worley.generate_noise();
        worley
    }

    fn generate_noise(&mut self) {
        let mut min_max = [i32::MAX, i32::MIN];

        for x in 0..self.resolution {
            for y in 0..self.resolution {
                for z in 0..self.resolution {
                    let id = UVec3::new(x as u32, y as u32, z as u32);
                    let pos = id.as_vec3() / self.resolution as f32;

                    let noise_sum = Self::worley(&self.points_a, self.num_points_a, pos, self.tile)
                        + Self::worley(&self.points_b, self.num_points_b, pos, self.tile)
                            * self.persistence
                        + Self::worley(&self.points_c, self.num_points_c, pos, self.tile)
                            * self.persistence
                            * self.persistence;

                    let noise_sum =
                        noise_sum / (1.0 + self.persistence + self.persistence * self.persistence);
                    let noise_sum = if self.invert_noise {
                        1.0 - noise_sum
                    } else {
                        noise_sum
                    };

                    let scaled_val = (noise_sum * 10_000_000.0) as i32;
                    min_max[0] = min_max[0].min(scaled_val);
                    min_max[1] = min_max[1].max(scaled_val);

                    self.texture3d[id] =
                        self.texture3d[id] * (1.0 - self.color_mask) + noise_sum * self.color_mask;
                }
            }
        }

        for x in 0..self.resolution {
            for y in 0..self.resolution {
                for z in 0..self.resolution {
                    let id = UVec3::new(x as u32, y as u32, z as u32);
                    let val = self.texture3d[id];
                    let min_val = min_max[0] as f32 / 10_000_000.0;
                    let max_val = min_max[1] as f32 / 10_000_000.0;
                    let normalized_val = (val - min_val) / (max_val - min_val);
                    self.texture3d[id] = self.texture3d[id] * (1.0 - self.color_mask)
                        + normalized_val * self.color_mask;
                }
            }
        }
    }

    pub fn get(&self, vec3: Vec3) -> Vec4 {
        self.texture3d.sample_level(vec3, 0)
    }

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
        for i in 0..27 {
            let adj_id = cell_id + OFFSETS[i];
            if adj_id.min_element() == -1 || adj_id.max_element() == num_cells {
                let wrapped_id = (adj_id + num_cells) % num_cells;
                let adj_cell_index =
                    wrapped_id.x + num_cells * (wrapped_id.y + wrapped_id.z * num_cells);
                let wrapped_point = points[adj_cell_index as usize];
                for j in 0..27 {
                    let sample_offset = sample_pos - (wrapped_point + OFFSETS[j].as_vec3());
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

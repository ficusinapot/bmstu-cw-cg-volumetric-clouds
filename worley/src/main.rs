use eframe::egui;
use eframe::egui::{ColorImage, TextureHandle};
use glam::{IVec3, UVec3, Vec3, Vec4};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::ops::{Index, IndexMut};

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

impl<T> IndexMut<UVec3> for RWTexture3D<T> {
    fn index_mut(&mut self, coords: UVec3) -> &mut Self::Output {
        &mut self.data
            [coords.z as usize * self.x * self.y + coords.y as usize * self.x + coords.x as usize]
    }
}

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

fn create_worley_points_buffer(rng: &mut impl Rng, num_cells: usize) -> Vec<Vec3> {
    let mut points = vec![Vec3::ZERO; num_cells * num_cells * num_cells];
    let cell_size = 1.0 / num_cells as f32;

    for x in 0..num_cells {
        for y in 0..num_cells {
            for z in 0..num_cells {
                points[x + num_cells * (y + z * num_cells)] =
                    Vec3::new(x as f32, y as f32, z as f32) * cell_size
                        + Vec3::new(rng.random(), rng.random(), rng.random()) * cell_size;
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
            let wrapped_id = adj_id + num_cells;
            let wrapped_id = wrapped_id % num_cells;
            let adj_cell_index =
                wrapped_id.x + num_cells * (wrapped_id.y + wrapped_id.z * num_cells);
            let wrapped_point = points[adj_cell_index as usize];
            // for j in 0..27 {
            //     let sample_offset = sample_pos - (wrapped_point + OFFSETS[j].as_vec3());
            //     min_sqrt_dist = min_sqrt_dist.min(sample_offset.dot(sample_offset));
            // }
        } else {
            let adj_cell_index = adj_id.x + num_cells * (adj_id.y + adj_id.z * num_cells);
            let sample_offset = sample_pos - points[adj_cell_index as usize];
            min_sqrt_dist = min_sqrt_dist.min(sample_offset.dot(sample_offset));
        }
    }

    min_sqrt_dist.sqrt()
}

fn cs_worley(
    id: UVec3,
    resolution: u32,
    num_cells_a: usize,
    num_cells_b: usize,
    num_cells_c: usize,
    points_a: &[Vec3],
    points_b: &[Vec3],
    points_c: &[Vec3],
    persistence: f32,
    invert_noise: bool,
    tile: f32,
    channel_mask: Vec4,
    min_max: &mut [i32],
    result: &mut RWTexture3D<Vec4>,
) {
    let pos = id.as_vec3() / resolution as f32;

    let noise_sum = worley(points_a, num_cells_a, pos, tile)
        + worley(points_b, num_cells_b, pos, tile) * persistence
        + worley(points_c, num_cells_c, pos, tile) * persistence * persistence;
    let noise_sum = noise_sum / (1.0 + persistence + persistence * persistence);

    let noise_sum = if invert_noise {
        1.0 - noise_sum
    } else {
        noise_sum
    };
    let scaled_val = (noise_sum * 10_000_000.0) as i32;

    min_max[0] = min_max[0].min(scaled_val);
    min_max[1] = min_max[1].max(scaled_val);

    result[id] = result[id] * (1.0 - channel_mask) + noise_sum * channel_mask;
}

fn worley_to_image_z_slice(
    z_slice: usize,
    resolution: u32,
    num_cells_a: usize,
    num_cells_b: usize,
    num_cells_c: usize,
    points_a: &[Vec3],
    points_b: &[Vec3],
    points_c: &[Vec3],
    persistence: f32,
    invert_noise: bool,
    tile: f32,
    channel_mask: Vec4,
    result: &mut RWTexture3D<Vec4>,
) -> ColorImage {
    let mut min_max = [i32::MAX, i32::MIN];

    for x in 0..resolution {
        for y in 0..resolution {
            let id = UVec3::new(x, y, z_slice as u32);
            cs_worley(
                id,
                resolution,
                num_cells_a,
                num_cells_b,
                num_cells_c,
                points_a,
                points_b,
                points_c,
                persistence,
                invert_noise,
                tile,
                channel_mask,
                &mut min_max,
                result,
            );
        }
    }

    let min_val = min_max[0] as f32 / 10_000_000.;
    let max_val = min_max[1] as f32 / 10_000_000.;

    let mut pixels = Vec::with_capacity((resolution * resolution) as usize);
    for x in 0..resolution {
        for y in 0..resolution {
            let id = UVec3::new(x, y, z_slice as u32);
            let val = result[id];
            let normalized_val = (val - min_val) / (max_val - min_val);

            let final_val = normalized_val * channel_mask + val * (1.0 - channel_mask);

            let val_scaled = final_val * 255.0;
            pixels.push([
                val_scaled.x as u8,
                val_scaled.y as u8,
                val_scaled.z as u8,
                val_scaled.w as u8,
            ]);
        }
    }

    ColorImage::from_rgba_unmultiplied([resolution as usize, resolution as usize], &pixels.concat())
}



pub struct WorleyTextureApp {
    num_points_a: usize,
    num_points_b: usize,
    num_points_c: usize,
    points_a: Vec<Vec3>,
    points_b: Vec<Vec3>,
    points_c: Vec<Vec3>,
    z_slice: f32,
    persistence: f32,
    invert: bool,
    texture: Option<TextureHandle>,
    result: RWTexture3D<Vec4>,
    seed: u64,
    r: bool,
    g: bool,
    b: bool,
    a: bool,
}

impl Default for WorleyTextureApp {
    fn default() -> Self {
        let texture = None;
        let resolution = 512;
        let result = RWTexture3D {
            data: vec![Vec4::ZERO; (resolution * resolution * resolution) as usize],
            x: resolution as usize,
            y: resolution as usize,
            z: resolution as usize,
        };
        let mut std_rng = StdRng::seed_from_u64(0);
        let points_a = create_worley_points_buffer(&mut std_rng, 5);
        let points_b = create_worley_points_buffer(&mut std_rng, 13);
        let points_c = create_worley_points_buffer(&mut std_rng, 18);
        Self {
            num_points_a: 5,
            num_points_b: 13,
            num_points_c: 18,
            z_slice: 0.0,
            points_a,
            points_b,
            points_c,
            invert: false,
            persistence: 0.5,
            texture,
            result,
            seed: 0,
            r: false,
            g: false,
            b: false,
            a: true,
        }
    }
}

impl eframe::App for WorleyTextureApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Worley Texture Generator");
            let resolution = 512;
            let mut std_rng = StdRng::seed_from_u64(self.seed);

            let resp1 =
                ui.add(egui::Slider::new(&mut self.num_points_a, 1..=100).text("Number of A"));
            let resp2 =
                ui.add(egui::Slider::new(&mut self.num_points_b, 1..=100).text("Number of B"));
            let resp3 =
                ui.add(egui::Slider::new(&mut self.num_points_c, 1..=100).text("Number of C"));
            let resp4 = ui.checkbox(&mut self.invert, "Invert");
            let resp5 = ui.add(egui::Slider::new(&mut self.z_slice, 0.0..=0.9999).text("Z Slice"));
            let resp6 =
                ui.add(egui::Slider::new(&mut self.persistence, 0.1..=3.0).text("Persistence"));

            let resp7 = ui.checkbox(&mut self.r, "r");
            let resp8 = ui.checkbox(&mut self.g, "g");
            let resp9 = ui.checkbox(&mut self.b, "b");
            let resp10 = ui.checkbox(&mut self.a, "a");

            if resp1.changed()
                || resp2.changed()
                || resp3.changed()
                || resp4.clicked()
                || resp5.changed()
                || resp6.changed()
                || resp7.changed()
                || resp8.changed()
                || resp9.changed()
                || resp10.changed()
            {
                if resp1.changed() {
                    self.points_a = create_worley_points_buffer(&mut std_rng, self.num_points_a);
                }

                if resp2.changed() {
                    self.points_b = create_worley_points_buffer(&mut std_rng, self.num_points_b);
                }

                if resp3.changed() {
                    self.points_c = create_worley_points_buffer(&mut std_rng, self.num_points_c);
                }

                let tile = 1.0;
                let a = (self.r, self.g, self.b, self.a);
                let a = (a.0 as i32, a.1 as i32, a.2 as i32, a.3 as i32);
                let a = (a.0 as f32, a.1 as f32 * 0.5, a.2 as f32, a.3 as f32);

                let image = worley_to_image_z_slice(
                    (self.z_slice * resolution as f32) as usize,
                    resolution,
                    self.num_points_a,
                    self.num_points_b,
                    self.num_points_c,
                    &self.points_a,
                    &self.points_b,
                    &self.points_c,
                    self.persistence,
                    self.invert,
                    tile,
                    Vec4::from(a),
                    &mut self.result,
                );

                self.texture = Some(ctx.load_texture("worley_texture", image, Default::default()));
            }
            if let Some(texture) = &self.texture {
                ui.image(texture);
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 700.0]),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Worley",
        native_options,
        Box::new(|_| Ok(Box::from(WorleyTextureApp::default()))),
    )
        .expect("TODO: panic message");
}

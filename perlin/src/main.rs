use eframe::egui;
use egui::{ColorImage, TextureHandle, TextureOptions, Ui};
use glam::{Vec3, Vec4};
use domain::object::objects::texture3d::{PerlinBuilder};
use domain::object::objects::texture3d::{Perlin};
use svg::Document;
use svg::node::element::Rectangle;
use std::fs::File;
use std::io::Write;

pub struct NoiseVisualizer {
    perlin: Perlin,
    slice_y: f32,
    texture: Option<TextureHandle>,
}

impl NoiseVisualizer {
    pub fn new(perlin: Perlin) -> Self {
        Self {
            perlin,
            slice_y: 0.0,
            texture: None,
        }
    }

    fn generate_slice_image(&self, resolution: usize) -> ColorImage {
        let mut pixels = Vec::with_capacity(resolution * resolution);
        let resolution = resolution * 2;
        for z in 0..resolution {
            for x in 0..resolution {
                let sample = self.perlin.sample_level(Vec3::new(
                    x as f32 / resolution as f32,
                    self.slice_y as f32,
                    z as f32 / resolution as f32,
                ));
                let color = Self::vec4_to_rgba(sample);
                pixels.push(color[0]);
                pixels.push(color[1]);
                pixels.push(color[2]);
                pixels.push(color[3]);
            }
        }

        ColorImage::from_rgba_unmultiplied([resolution, resolution], &pixels)
    }

    fn vec4_to_rgba(vec: Vec4) -> [u8; 4] {
        [
            (vec.x.clamp(0.0, 1.0) * 255.0) as u8,
            (vec.y.clamp(0.0, 1.0) * 255.0) as u8,
            (vec.z.clamp(0.0, 1.0) * 255.0) as u8,
            (vec.w.clamp(0.0, 1.0) * 255.0) as u8,
        ]
    }

    pub fn export_to_svg(&self, resolution: usize) {
        let resolution = resolution;
        let mut document = Document::new()
            .set("viewBox", (0, 0, resolution, resolution))
            .set("xmlns", "http://www.w3.org/2000/svg");

        for z in 0..resolution {
            for x in 0..resolution {
                let sample = self.perlin.sample_level(Vec3::new(
                    x as f32 / resolution as f32,
                    self.slice_y as f32,
                    z as f32 / resolution as f32,
                ));
                let color = Self::vec4_to_rgba(sample);
                let rect = Rectangle::new()
                    .set("x", x)
                    .set("y", z)
                    .set("width", 1)
                    .set("height", 1)
                    .set("fill", format!("rgb({}, {}, {})", color[0], color[1], color[2]));
                document = document.add(rect);
            }
        }

        let svg_data = document.to_string();
        let mut file = File::create("perlin.svg").expect("Unable to create file");
        file.write_all(svg_data.as_bytes()).expect("Unable to write data to file");
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("Perlin Noise Visualizer");

        ui.horizontal(|ui| {
            let resp = ui.add(egui::Slider::new(&mut self.slice_y, 0.0..=1.0));
            ui.label(format!("Slice Y: {}", self.slice_y));

            let resolution = self.perlin.builder.resolution;
            if self.texture.is_none() || resp.changed() {
                let image = self.generate_slice_image(resolution);
                self.texture = Some(ui.ctx().load_texture(
                    "perlin_slice",
                    image,
                    TextureOptions::LINEAR,
                ));
            }
        });

        ui.collapsing("Noise Settings", |ui| {
            let params = &mut self.perlin.builder;

            ui.add(egui::Slider::new(&mut params.resolution, 16..=256).text("Resolution"));
            ui.add(egui::Slider::new(&mut params.num_points_a, 1..=64).text("Num Points A"));
            ui.add(egui::Slider::new(&mut params.num_points_b, 1..=64).text("Num Points B"));
            ui.add(egui::Slider::new(&mut params.num_points_c, 1..=64).text("Num Points C"));
            ui.add(egui::Slider::new(&mut params.persistence, 0.0..=1.0).text("Persistence"));
            ui.add(egui::Slider::new(&mut params.tile, 0.1..=10.0).text("Tile"));
            ui.add(egui::Checkbox::new(&mut params.invert_noise, "invert"));

            if ui.button("Regenerate Noise").clicked() {
                self.perlin = Perlin::build(params.clone());
                self.texture = None;
            }

            if ui.button("Export to SVG").clicked() {
                self.export_to_svg(self.perlin.builder.resolution);
            }
        });

        if let Some(texture) = &self.texture {
            ui.image(texture);
        }
    }
}

fn main() {
    let perlin = PerlinBuilder::default()
        .with_seed(0)
        .with_resolution(128)
        .with_num_points_a(3)
        .with_num_points_b(7)
        .with_num_points_c(11)
        .with_persistence(0.0)
        .with_tile(1.0)
        .with_color_mask(Vec4::new(1.0, 1.0, 1.0, 1.0))
        .with_invert_noise(true)
        .build();

    let app = NoiseApp {
        visualizer: NoiseVisualizer::new(perlin),
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Perlin Noise Visualizer",
        options,
        Box::new(|_| Ok(Box::from(app))),
    )
        .expect("TODO: panic message");
}

struct NoiseApp {
    visualizer: NoiseVisualizer,
}

impl eframe::App for NoiseApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.visualizer.ui(ui);
        });
    }
}

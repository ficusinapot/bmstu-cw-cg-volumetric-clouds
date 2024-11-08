use eframe::egui;
use eframe::egui::Color32;

use domain::canvas::painter::Painter3D;
use domain::facade::{CameraCommand, DrawCommand, SceneCommand};
use domain::facade::Facade;
use domain::math::transform::glam::{Vec3, Vec4};
use domain::object::camera::Camera;
use domain::object::objects::{Cloud, Grid};
use domain::object::objects::texture3d::WorleyBuilder;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

impl App {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(1024.0);
                ui.set_height(800.0);
                let (resp, painter) = self.painter(ui);
                self.executor.exec(DrawCommand::SetPainter(painter));
                self.handle_camera(&resp, ui);
                self.executor.exec(DrawCommand::Draw);
            });
            self.control(ui);
        });
    }

    fn painter(&self, ui: &mut egui::Ui) -> (egui::Response, Painter3D) {
        let (response, painter) =
            ui.allocate_painter([1056.0, 900.0].into(), egui::Sense::click_and_drag());
        painter.rect(
            painter.clip_rect().shrink(0.0),
            0.0,
            Color32::WHITE,
            egui::Stroke::new(0.5, egui::Color32::BLACK),
        );
        let rect = response.rect;
        (response, Painter3D::new(painter, rect))
    }

    fn handle_camera(&mut self, resp: &egui::Response, ui: &mut egui::Ui) {
        if resp.dragged_by(egui::PointerButton::Primary) {
            if ui.input(|i| i.raw.modifiers.shift_only()) {
                let pan = CameraCommand::Pan(resp.drag_delta().x, resp.drag_delta().y);
                self.executor.exec(pan);
            } else {
                let pivot = CameraCommand::Pivot(resp.drag_delta().x, resp.drag_delta().y);
                self.executor.exec(pivot);
            }
        }

        if resp.dragged_by(egui::PointerButton::Secondary) {
            let pan = CameraCommand::Pan(resp.drag_delta().x, resp.drag_delta().y);
            self.executor.exec(pan);
        }

        if resp.hovered() {
            let zoom = CameraCommand::Zoom(ui.input(|i| -i.raw_scroll_delta.y));
            self.executor.exec(zoom);
        }
    }

    fn control(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.collapsing("Параметры облаков", |ui| {
                ui.vertical(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(&mut self.num_steps, 1..=1000)
                                    .drag_value_speed(0.05),
                            );
                            ui.label("num_steps");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetNumSteps("cloud", self.num_steps));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(&mut self.cloud_scale, 1.0..=1000.0)
                                    .drag_value_speed(0.05),
                            );
                            ui.label("cloud_scale");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetCloudScale("cloud", self.cloud_scale));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.density_multiplier,
                                    1.0..=20.0,
                                )
                                .drag_value_speed(0.05),
                            );
                            ui.label("Плотность");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetDensityMultiplier(
                                    "cloud",
                                    self.density_multiplier,
                                ));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(&mut self.density_threshold, 0.0..=1.0)
                                    .drag_value_speed(0.001),
                            );
                            ui.label("Предел плотности");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetDensityThreshold(
                                    "cloud",
                                    self.density_threshold,
                                ));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(egui::widgets::Slider::new(
                                &mut self.offset.x,
                                -1000.0..=1000.0,
                            ));
                            ui.label("offset_x");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetOffset("cloud", self.offset));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(&mut self.offset.y, -1000.0..=1000.0), // .drag_value_speed(0.001),
                            );
                            ui.label("offset_y");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetOffset("cloud", self.offset));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(&mut self.offset.z, -1000.0..=1000.0)
                                    .drag_value_speed(0.001),
                            );
                            ui.label("offset_z");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetOffset("cloud", self.offset));
                            }
                        });
                    });

                    ui.collapsing("Шум Вороного", |ui| {
                        ui.horizontal(|ui| {
                            ui.add(egui::widgets::DragValue::new(&mut self.worley_builder.seed));
                            ui.label("Сид");
                        });

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.worley_builder.persistence,
                                    0.0..=1.0,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("Постоянство");
                        });

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.worley_builder.num_points_a,
                                    1..=30,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("A");
                        });

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.worley_builder.num_points_b,
                                    1..=30,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("B");
                        });

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.worley_builder.num_points_c,
                                    1..=30,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("C");
                        });

                        ui.horizontal(|ui| {
                            ui.add(egui::widgets::Checkbox::new(
                                &mut self.worley_builder.invert_noise,
                                "Инверсия шума",
                            ));
                        });

                        if ui.button("Сгенерировать").clicked() {
                            self.executor
                                .exec(SceneCommand::SetNoise("cloud", self.worley_builder));
                        }
                    });
                });
            });
        });
    }
}

pub fn init_app() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([2880.0, 1920.0]),
        centered: true,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Курсовая работа",
        native_options,
        Box::new(|_| Ok(Box::from(App::init()))),
    )
}

struct App {
    executor: Facade,
    num_steps: usize,
    cloud_scale: f32,
    density_multiplier: f32,
    density_threshold: f32,
    alpha_threshold: u8,
    offset: Vec3,

    worley_builder: WorleyBuilder,
}

impl App {
    fn init() -> Self {
        let num_steps = 50;
        let cloud_scale = 100.0;
        let density_multiplier = 5.0;
        let density_threshold = 0.57;
        let alpha_threshold = 255;
        let seed = 0;
        let num_points_a = 29;
        let num_points_b = 7;
        let num_points_c = 7;
        let tile = 1.0;
        let resolution = 128;
        let persistence = 0.6;
        let invert_noise = true;
        let color_mask = Vec4::splat(1.0);
        let worley = WorleyBuilder::new()
            .with_seed(seed)
            .with_num_points_a(num_points_a)
            .with_num_points_b(num_points_b)
            .with_num_points_c(num_points_c)
            .with_tile(tile)
            .with_resolution(resolution)
            .with_color_mask(color_mask)
            .with_persistence(persistence)
            .with_invert_noise(invert_noise);

        let mut executor = Facade::default();
        executor.exec(CameraCommand::SetCamera(Camera::default()));
        executor.exec(SceneCommand::AddObject(
            "cloud",
            Cloud::new((Vec3::new(-1., 1.0, -1.), Vec3::new(1., 1.5, 1.)))
                .with_clouds_offset(Vec3::ZERO)
                .with_cloud_scale(cloud_scale)
                .with_density_threshold(density_threshold)
                .with_density_multiplier(density_multiplier)
                .with_num_steps(num_steps)
                .with_alpha_threshold(alpha_threshold)
                .with_color(Color32::WHITE)
                .with_noise(worley)
                .into(),
        ));
        executor.exec(SceneCommand::AddObject("grid", Grid::new(10, 1.0).into()));
        Self {
            executor,
            num_steps,
            cloud_scale,
            density_multiplier,
            density_threshold,
            offset: Vec3::ZERO,
            worley_builder: worley,
            alpha_threshold,
        }
    }
}

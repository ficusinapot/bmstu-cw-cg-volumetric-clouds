use domain::canvas::painter::Painter3D;
use domain::facade::Facade;
use domain::facade::{CameraCommand, DrawCommand, SceneCommand};
use domain::math::transform::glam::Vec3;
use domain::object::camera::Camera;
use domain::object::objects::{Cloud, Grid};
use eframe::egui;
use eframe::egui::Color32;
use std::time::Duration;

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
                    egui::widgets::Slider::new(&mut self.density_multiplier, 1.0..=20.0)
                        .drag_value_speed(0.05),
                );
                ui.label("density_multiplier");
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
                ui.label("density_threshold");
                if resp.changed() {
                    self.executor.exec(SceneCommand::SetDensityThreshold(
                        "cloud",
                        self.density_threshold,
                    ));
                }
            });

            ui.horizontal(|ui| {
                let resp = ui.add(
                    egui::widgets::Slider::new(&mut self.alpha_threshold, 0..=255)
                        .drag_value_speed(1.0),
                );
                ui.label("alpha_threshold");
                if resp.changed() {
                    self.executor.exec(SceneCommand::SetAlphaThreshold(
                        "cloud",
                        self.alpha_threshold,
                    ));
                }
            });

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let resp = ui.add(
                        egui::widgets::Slider::new(&mut self.offset.x, -50.0..=50.0)
                            .drag_value_speed(0.1),
                    );
                    ui.label("offset_x");
                    if resp.changed() {
                        self.executor
                            .exec(SceneCommand::SetOffset("cloud", self.offset));
                    }
                });
                ui.horizontal(|ui| {
                    let resp = ui.add(
                        egui::widgets::Slider::new(&mut self.offset.y, -50.0..=50.0)
                            .drag_value_speed(0.1),
                    );
                    ui.label("offset_y");
                    if resp.changed() {
                        self.executor
                            .exec(SceneCommand::SetOffset("cloud", self.offset));
                    }
                });
                ui.horizontal(|ui| {
                    let resp = ui.add(
                        egui::widgets::Slider::new(&mut self.offset.z, -50.0..=50.0)
                            .drag_value_speed(0.1),
                    );
                    ui.label("offset_z");
                    if resp.changed() {
                        self.executor
                            .exec(SceneCommand::SetOffset("cloud", self.offset));
                    }
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
}

impl App {
    fn init() -> Self {
        let mut executor = Facade::default();
        executor.exec(CameraCommand::SetCamera(Camera::default()));
        executor.exec(SceneCommand::AddObject(
            "cloud",
            Cloud::new((Vec3::new(-1.0, 1.0, -1.0), Vec3::new(1.0, 1.5, 1.0))).into(),
        ));
        executor.exec(SceneCommand::AddObject("grid", Grid::new(10, 1.0).into()));
        Self {
            executor,
            num_steps: 17,
            cloud_scale: 69.0,
            density_multiplier: 5.,
            density_threshold: 0.57,
            offset: Vec3::ZERO,
            alpha_threshold: 255
        }
    }
}

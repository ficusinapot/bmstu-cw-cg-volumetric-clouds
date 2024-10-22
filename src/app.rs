use eframe::egui;
use domain::canvas::painter::Painter3D;
use domain::facade::{DrawCommand, CameraCommand, SceneCommand};
use domain::facade::Facade;
use domain::math::transform::glam::Vec3;
use domain::object::camera::Camera;
use domain::object::objects::{Grid, Cloud};

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
                let resp = ui.allocate_response([1056.0, 900.0].into(), egui::Sense::click_and_drag());
                let painter = self.painter(&resp, ui);
                self.executor.exec(DrawCommand::SetPainter(painter));
                self.handle_camera(&resp, ui);
                self.executor.exec(DrawCommand::Draw);
            });
            self.control(ui);
        });
    }

    fn painter(&self, resp: &egui::Response, ui: &mut egui::Ui) -> Painter3D {
        Painter3D::new(ui.painter_at(resp.rect), resp.rect)
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
            ui.vertical_centered_justified(|ui| {
                let _ = ui.button("Test");
            });
        });
    }
}

pub fn init_app() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([2880.0, 1920.0]),
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
}

impl App {
    fn init() -> Self {
        let mut executor = Facade::default();
        executor.exec(SceneCommand::AddObject(Grid::new(10, 1.0).into()));
        executor.exec(CameraCommand::SetCamera(Camera::default()));
        executor.exec(SceneCommand::AddObject(Cloud::new(
                (Vec3::new(-1.0, 1.0, -1.0), Vec3::new(1.0, 1.5, 1.0))
        ).into()));
        Self {
            executor
        }
    }
}

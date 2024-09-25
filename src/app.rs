use eframe::egui;
use eframe::egui::Color32;
use core::glam::Vec3;
use core::utils;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            core::threegui(ui, [1024.0, 800.0], |three| {
                let paint = three.painter();
                utils::grid(paint, 10, 1.0, egui::Stroke::new(1.0, Color32::GRAY));
                paint.circle_filled(Vec3::new(10.0, 10.0, 10.0), 10.0, Color32::YELLOW);
            })
        });
    }
}

pub fn init_app() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((1440.0, 960.0)),
        default_theme: eframe::Theme::Light,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Курсовая работа",
        native_options,
        Box::new(|_| Ok(Box::<App>::default())),
    )
}

#[derive(Default, Debug)]
struct App {
    // executor: Facade
}

impl App {
}

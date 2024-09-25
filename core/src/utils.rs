use eframe::epaint::Color32;
use egui::Stroke;
pub use glam::Vec3;

use crate::Painter3D;

pub fn grid(paint: &Painter3D, k: i32, scale: f32, stroke: Stroke) {
    let f = k as f32;
    for i in -k..=k {
        paint.line(
            Vec3::new(-1., 0., i as f32 / f) * scale,
            Vec3::new(1., 0., i as f32 / f) * scale,
            stroke,
        );

        paint.line(
            Vec3::new(i as f32 / f, 0., -1.) * scale,
            Vec3::new(i as f32 / f, 0., 1.) * scale,
            stroke,
        );
    }

    paint.line(
        Vec3::new(0., 0., 0.) * scale,
        Vec3::new(0., 1.5, 0.) * scale,
        Stroke::new(2.0, Color32::BLUE),
    );

    paint.line(
        Vec3::new(0., 0., 0.) * scale,
        Vec3::new(0., 0., -1.5) * scale,
        Stroke::new(2.0, Color32::RED),
    );

    paint.line(
        Vec3::new(0., 0., 0.) * scale,
        Vec3::new(-1.5, 0., 0.) * scale,
        Stroke::new(2.0, Color32::DARK_GREEN),
    );
}

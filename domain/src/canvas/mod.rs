pub mod painter;

use egui::Stroke;
use glam::Vec3;

pub trait Canvas {
    fn line(&self, a: Vec3, b: Vec3, stroke: Stroke);
    fn arrow(&self, pos: Vec3, dir: Vec3, screen_len: f32, stroke: Stroke);
    // fn circle_filled(&self, center: Vec3, radius: f32, fill_color: impl Into<Color32>);
    // fn circle(&self, center: Vec3, radius: f32, stroke: impl Into<Stroke>);
}


// impl Canvas for Painter3D {
//     fn render(&self) {
//         self.show(self)
//     }
// }


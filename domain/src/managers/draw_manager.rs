use egui::{Color32, Stroke, TextureId};

use crate::canvas::painter::Painter3D;
use crate::managers::Manager;
use crate::object::camera::Camera;
use crate::object::objects::Sun;
use crate::scene::scene::Scene;
use crate::visitor::draw_visitor::DrawVisitor;
use crate::visitor::Visitable;

#[derive(Clone, Default)]
pub struct DrawManager {
    canvas: Option<Painter3D>,
    stroke: Stroke,
    color: Color32,
}

impl DrawManager {
    pub fn set_canvas(&mut self, canvas: Painter3D) {
        self.canvas = Option::from(canvas);
    }

    pub fn set_stroke(&mut self, stroke: Stroke) {
        self.stroke = stroke;
    }

    pub fn set_color(&mut self, color: Color32) {
        self.color = color;
    }

    pub fn draw_scene(&self, scene: &Scene, camera: &Camera) {
        if let Some(canvas) = &self.canvas {
            let mut img = egui::ColorImage::new([1056, 900], self.color);
            let mut visitor = DrawVisitor::new(&mut img, camera, canvas)
                .with_color(self.color)
                .with_stroke(self.stroke);

            scene.accept(&mut visitor);
            // let handle = canvas.ctx()
            //     .load_texture("worley_texture", img, Default::default());
            // let textureid = TextureId::from(&handle);
            // canvas.image(
            //     textureid,
            //     egui::Rect::from_two_pos((0.0, 0.0).into(), (1056.0, 900.0).into()),
            //     egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            //     Color32::WHITE,
            // );
        }

       
    }
}

impl Manager for DrawManager {}

use egui::{Color32, Stroke};

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

    pub fn draw_scene(&self, scene: &Scene, camera: &Camera, sun: &Sun) {
        if self.canvas.is_none() {
            return;
        }
        let canvas = self.canvas.as_ref().expect("");
        let visitor = DrawVisitor::new(canvas, camera)
            .with_sun(sun)
            .with_color(self.color)
            .with_stroke(self.stroke);
        scene.accept(&visitor);
    }
}

impl Manager for DrawManager {}

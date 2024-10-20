use egui::Stroke;
use crate::managers::Manager;
use crate::object::camera::Camera;
use crate::scene::scene::Scene;
use crate::visitor::draw_visitor::DrawVisitor;
use crate::visitor::Visitable;
use crate::canvas::painter::Painter3D;

#[derive(Clone, Default)]
pub struct DrawManager {
    canvas: Option<Painter3D>,
    stroke: Stroke,
}

impl DrawManager {
    pub fn set_canvas(&mut self, canvas: Painter3D) {
        self.canvas = Option::from(canvas);
    }

    pub fn set_stroke(&mut self, stroke: Stroke) {
        self.stroke = stroke;
    }

    pub fn draw_scene(&self, scene: &Scene, camera: &Camera) {
        if self.canvas.is_none() {
            return;
        }
        let canvas = self.canvas.as_ref().expect("");
        let visitor = DrawVisitor::new(canvas, camera);
        scene.accept(&visitor);
    }
}

impl Manager for DrawManager {}

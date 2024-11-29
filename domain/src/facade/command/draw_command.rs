use crate::canvas::painter::Painter3D;
use crate::facade::Command;
use crate::managers::ManagerSolution;
use crate::object::Component;

pub enum DrawCommand {
    SetPainter(Painter3D),
    SetPainterColor(egui::Color32),
    Draw,
}

impl Command for DrawCommand {
    type ReturnType = ();
    fn exec(self, manager: &mut ManagerSolution) {
        match self {
            Self::SetPainter(painter) => {
                let dm = manager.get_mut_draw_manager();
                dm.set_canvas(painter)
            }
            Self::SetPainterColor(color) => {
                let dm = manager.get_mut_draw_manager();
                dm.set_color(color);
            }
            Self::Draw => {
                let draw = manager.get_draw_manager();
                let camera = manager.get_camera_manager().get_camera();
                let scene = manager.get_scene_manager().get_scene();

                let sun = manager.get_scene_manager().get_scene().get_object("sun");
                if let Some(Component::Sun(sun)) = sun {
                    draw.draw_scene(scene, camera, sun)
                }
            }
        }
    }
}

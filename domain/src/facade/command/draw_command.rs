use crate::canvas::painter::Painter3D;
use crate::facade::Command;
use crate::managers::ManagerSolution;

pub enum DrawCommand {
    SetPainter(Painter3D),
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
            Self::Draw => {
                let draw = manager.get_draw_manager();
                let camera = manager.get_camera_manager().get_camera();
                let scene = manager.get_scene_manager().get_scene();
                draw.draw_scene(scene, camera)
            }
        }
    }
}

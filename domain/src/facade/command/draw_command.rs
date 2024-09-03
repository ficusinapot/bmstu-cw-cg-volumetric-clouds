use crate::facade::Command;
use crate::managers::ManagerSolution;
use log::debug;

#[derive(Debug)]
pub enum DrawCommandKind {
    Draw,
}

impl Command for DrawCommandKind {
    fn exec(self, manager: &mut ManagerSolution) {
        let command: DrawCommand = self.into();
        command.exec(manager);
    }
}

#[derive(Debug)]
pub struct DrawCommand {
    kind: DrawCommandKind,
}

impl DrawCommand {
    pub fn new(kind: DrawCommandKind) -> Self {
        Self { kind }
    }
}

impl Command for DrawCommand {
    fn exec(self, manager: &mut ManagerSolution) {
        debug!("Executing {:?}", self);
        match self.kind {
            DrawCommandKind::Draw => {
                let camera = manager.get_camera_manager().get_camera();
                let scene = manager.get_scene_manager().get_scene();
                manager.draw_manager.draw_scene(camera, scene);
            }
        }
    }
}

impl From<DrawCommandKind> for DrawCommand {
    fn from(value: DrawCommandKind) -> Self {
        Self::new(value)
    }
}

mod config;

use crate::config::init_logger;
use domain::facade::{DrawCommand, DrawCommandKind, Facade, SceneCommandKind};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    init_logger()?;
    let mut facade = Facade::default();
    let cc = SceneCommandKind::AddObject;
    facade.exec(cc);
    let cc = DrawCommand::new(DrawCommandKind::Draw);
    facade.exec(cc);
    Ok(())
}

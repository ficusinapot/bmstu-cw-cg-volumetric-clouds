mod config;

use domain::facade::{SceneCommandKind, Facade, SceneCommand, DrawCommand, DrawCommandKind};
use std::error::Error;
use crate::config::init_logger;

fn main() -> Result<(), Box<dyn Error>> {
    init_logger()?;
    let mut facade = Facade::default();
    let cc = SceneCommand::new(SceneCommandKind::AddObject);
    facade.exec(cc);
    let cc = DrawCommand::new(DrawCommandKind::Draw);
    facade.exec(cc);
    
    Ok(())
}

mod config;

use domain::facade::{CameraCommand, CameraCommandKind, Facade, SceneCommand};
use std::error::Error;
use crate::config::init_logger;

fn main() -> Result<(), Box<dyn Error>> {
    init_logger()?;
    let mut facade = Facade::default();
    
    let cc = SceneCommand::new();
    facade.exec(cc);
    let cc = SceneCommand::new();
    facade.exec(cc);
    Ok(())
}

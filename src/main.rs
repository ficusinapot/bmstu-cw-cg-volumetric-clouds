mod app;
mod config;

use app::init_app;

use crate::config::init_logger;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    init_logger()?;
    init_app()?;
    Ok(())
}

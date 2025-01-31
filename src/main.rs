mod app;
mod config;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init_logger()?;
    config::init_rayon()?;
    app::init_app()?;
    Ok(())
}

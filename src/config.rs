use log::debug;
use rayon::{ThreadPoolBuilder, ThreadPoolBuildError};

#[allow(clippy::pedantic)]

pub fn init_logger() -> Result<(), log::SetLoggerError> {
    if std::env::var_os("RUST_LOG").is_none() {
        env_logger::builder()
            .filter(None, log::LevelFilter::Info)
            .filter(None, log::LevelFilter::Debug)
            .format_timestamp(None)
            .try_init()?;
    } else {
        env_logger::try_init()?;
    }
    debug!("init_logger: Ok");
    Ok(())
}

pub fn init_rayon() -> Result<(), ThreadPoolBuildError> {
   ThreadPoolBuilder::new().num_threads(16).build_global()
}
use domain::facade::{Facade, ObjectCommand};
use log::debug;

#[allow(clippy::pedantic)]

pub(crate) fn init_logger() -> Result<(), log::SetLoggerError> {
    if std::env::var_os("RUST_LOG").is_none() {
        env_logger::builder()
            .default_format()
            .filter(None, log::LevelFilter::Info)
            .filter(None, log::LevelFilter::Debug)
            .try_init()?;
    } else {
        env_logger::try_init()?;
    }
    debug!("init_logger: Ok");
    Ok(())
}

fn main() -> Result<(), Box<log::SetLoggerError>> {
    init_logger()?;
    let oc = ObjectCommand::new();
    Facade::exec(oc);
    Ok(())
}

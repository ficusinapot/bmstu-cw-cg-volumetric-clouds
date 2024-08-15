pub mod command;
#[allow(clippy::module_inception)]
pub mod facade;

pub use command::{Command, ObjectCommand};

pub use facade::Facade;

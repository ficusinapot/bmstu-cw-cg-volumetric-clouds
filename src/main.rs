mod domain;
use domain::facade::{Facade, ObjectCommand};

#[allow(clippy::pedantic)]

fn main() {
    let oc = ObjectCommand::new();
    Facade::exec(oc);
}

/// Вывод в stdout вместо stderr
use env_logger::{Builder, Target};
use log::error;

fn main() {
    Builder::new().target(Target::Stdout).init();

    error!("This error has been printed to Stdout");
}

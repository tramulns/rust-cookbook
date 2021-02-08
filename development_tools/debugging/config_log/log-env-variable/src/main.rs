/// Использование переменной среды для настройки логирования
/// MY_APP_LOG=info cargo run
use env_logger::Builder;

fn main() {
    let mut builder = Builder::from_env("MY_APP_LOG");
    builder.init();

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");
}

/// Включить метку времени в сообщения лога
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");
}

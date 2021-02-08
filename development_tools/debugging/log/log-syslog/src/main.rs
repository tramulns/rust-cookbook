/// Логирование в Unix syslog
/// sudo journalctl | grep "My app name"
use syslog;

use syslog::{Error, Facility};

fn main() -> Result<(), Error> {
    syslog::init(
        Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("My app name"),
    )?;
    log::debug!("this is a debug {}", "message");
    log::error!("this is an error!");
    Ok(())
}

mod class_logger;
mod colors;
mod log_level;
mod logger;

use std::sync::OnceLock;

pub use class_logger::Logger;
pub use log_level::LogLevel;
pub use logger::__log;

pub(crate) fn global_log_level() -> &'static LogLevel {
    static LOG_LEVEL: OnceLock<LogLevel> = OnceLock::new();
    let level = match std::env::var("LOG_LEVEL") {
        Ok(var) => LOG_LEVEL.get_or_init(|| LogLevel::from(var.as_str())),
        Err(_) => LOG_LEVEL.get_or_init(|| LogLevel::INFO),
    };
    level
}

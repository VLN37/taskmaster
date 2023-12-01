use crate::logger::current_time;
use crate::{global_log_level, LogLevel};

#[derive(Debug)]
pub struct Logger {
    pub module:    String,
    pub log_level: LogLevel,
}

impl Logger {
    pub fn new(module: &str) -> Logger {
        Logger {
            module:    module.into(),
            log_level: global_log_level().clone(),
        }
    }

    fn log(&self, log_level: LogLevel, msg: &str) {
        if self.log_level >= *global_log_level() {
            println!("[{}][{:5}] {}", current_time(), log_level, msg);
        }
    }

    pub fn debug(&self, msg: &str) { self.log(LogLevel::DEBUG, msg) }
    pub fn info(&self, msg: &str) { self.log(LogLevel::INFO, msg) }
    pub fn warn(&self, msg: &str) { self.log(LogLevel::WARN, msg) }
    pub fn error(&self, msg: &str) { self.log(LogLevel::ERROR, msg) }
}

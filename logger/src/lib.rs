use std::env;
use std::ffi::CString;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Logger {
    pub module:    String,
    pub log_level: LogLevel,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LogLevel {
    ERROR,
    WARN,
    INFO,
    DEBUG,
}

impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        match value {
            "DEBUG" => LogLevel::DEBUG,
            "INFO" => LogLevel::INFO,
            "WARN" => LogLevel::WARN,
            "WARNING" => LogLevel::WARN,
            "ERROR" => LogLevel::ERROR,
            _ => panic!("invalid"),
        }
    }
}

impl LogLevel {
    fn global_log_level() -> &'static LogLevel {
        static LOG_LEVEL: OnceLock<LogLevel> = OnceLock::new();
        match env::var("LOG_LEVEL") {
            Ok(var) => return LOG_LEVEL.get_or_init(|| LogLevel::from(var.as_str())),
            Err(_) => return LOG_LEVEL.get_or_init(|| LogLevel::INFO),
        }
    }
}

impl Logger {
    pub fn new(module: &str) -> Logger {
        Logger {
            module:    module.into(),
            log_level: LogLevel::global_log_level().clone(),
        }
    }

    pub fn debug(&self, msg: &str) {
        if self.log_level >= LogLevel::DEBUG {
            println!("[{}][{:5}] {}: {}", current_time(), "DEBUG", self.module, msg);
        }
    }

    pub fn info(&self, msg: &str) {
        if self.log_level >= LogLevel::INFO {
            println!("[{}][{:5}] {}: {}", current_time(), "INFO", self.module, msg);
        }
    }

    pub fn warn(&self, msg: &str) {
        if self.log_level >= LogLevel::WARN {
            println!("[{}][{:5}] {}: {}", current_time(), "WARN", self.module, msg);
        }
    }

    pub fn error(&self, msg: &str) {
        if self.log_level >= LogLevel::ERROR {
            println!("[{}][{:5}] {}: {}", current_time(), "ERROR", self.module, msg);
        }
    }
}

fn current_time() -> String {
    unsafe {
        let mut buf: [libc::c_char; 50] = [0; 50];
        let raw_time = libc::time(std::ptr::null_mut::<libc::time_t>());
        let tm = libc::localtime(&raw_time);
        let format = CString::new("%d/%m/%Y %H:%M:%S").expect("to work");

        let borrow = format.into_raw();
        let i = libc::strftime(buf.as_mut_ptr(), 50, borrow, tm);
        let _retake = CString::from_raw(borrow);

        let s = String::from_utf8(buf[..i].iter().map(|&x| x as u8).collect()).unwrap();
        s
    }
}

use std::ffi::CString;
use std::io::IsTerminal;

use crate::colors::Colors;
use crate::{global_log_level, LogLevel};

#[doc(hidden)]
pub fn __log(log_level: LogLevel, file: &str, msg: &str) {
    let istty = std::io::stdout().is_terminal();
    let mut time = format!("[{}]", current_time());
    let mut lvl = format!("[{log_level:5}]");
    let mut file_str = file[0..file.find('/').unwrap_or(file.len())].to_string();
    if istty {
        time = format!("{}{time}{}", Colors::LightGreen, Colors::Reset);
        lvl = format!("{}{lvl}{}", log_level.color(), Colors::Reset);
        file_str = format!("{}{file_str}{}", Colors::LightCyan, Colors::Reset);
    }
    if log_level >= *global_log_level() {
        println!("{time}{lvl} {file_str:>8}: {msg}",);
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        logger::__log(logger::LogLevel::DEBUG, file!(), &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        logger::__log(logger::LogLevel::INFO, file!(), &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        logger::__log(logger::LogLevel::WARN, file!(), &format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        logger::__log(logger::LogLevel::ERROR, file!(), &format!($($arg)*));
    }};
}

pub(crate) fn current_time() -> String {
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

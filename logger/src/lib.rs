use std::ffi::CString;

pub struct Logger {
    module: String,
}

impl Logger {
    pub fn new(module: &str) -> Logger {
        Logger {
            module: module.into(),
        }
    }

    pub fn debug(&self, msg: &str) {
        println!("[{}][{:5}] {}: {}", current_time(), "DEBUG", self.module, msg)
    }

    pub fn info(&self, msg: &str) {
        println!("[{}][{:5}] {}: {}", current_time(), "INFO", self.module, msg)
    }

    pub fn warn(&self, msg: &str) {
        println!("[{}][{:5}] {}: {}", current_time(), "WARN", self.module, msg)
    }

    pub fn error(&self, msg: &str) {
        println!("[{}][{:5}] {}: {}", current_time(), "ERROR", self.module, msg)
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

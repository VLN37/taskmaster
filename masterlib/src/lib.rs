pub static SOCKET_PATH: &str = "taskmaster-socket";
pub static CONFIG_PATH: &str = "config.yml";

pub mod common;
pub mod daemon;
syscall!(libc::epollcreate())

#![feature(raw_os_error_ty)]

mod cmd;
pub mod macros;
pub mod server;

pub use cmd::{Cmd, CmdError};

pub const DAEMON_SOCKET_PATH: &str = "/tmp/daemon.sock";
pub const CTL_SOCKET_PATH: &str = "/tmp/ctl.sock";

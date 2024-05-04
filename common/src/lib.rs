#![feature(raw_os_error_ty)]

mod cmd;
mod macros;
pub mod request;
pub mod response;
pub mod server;

pub use cmd::{Cmd, CmdError, CmdErrorKind, CmdHandler};
pub use request::{ClientState, Request, RequestError, RequestStatus};
pub use response::Response;
pub use server::Key;

pub const DAEMON_SOCKET_PATH: &str = "/tmp/daemon.sock";
pub const CTL_SOCKET_PATH: &str = "/tmp/ctl.sock";

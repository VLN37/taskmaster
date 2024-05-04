mod class;
mod error;
mod error_kind;

use std::result;

pub use class::Cmd;
pub use error::CmdError;
pub use error_kind::CmdErrorKind;

use crate::request::Request;

pub type Result<Cmd> = result::Result<Cmd, CmdError>;

pub trait CmdHandler {
    fn handle(&mut self, request: &mut Request) -> result::Result<String, CmdError>;
    fn attach(&mut self, request: &mut Request) -> result::Result<String, CmdError>;
    fn log(&self, request: &mut Request) -> result::Result<String, CmdError>;
    fn head(&self, request: &mut Request) -> result::Result<String, CmdError>;
    fn status(&self, request: &mut Request) -> result::Result<String, CmdError>;
    fn other(&self, request: &mut Request) -> result::Result<String, CmdError>;
}

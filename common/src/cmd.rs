use core::fmt;
use std::result;

use crate::request::Request;

#[derive(Debug, PartialEq, Clone)]
pub enum Cmd {
    Log,
    Status,
    Head,
    Attach,
    Other(String),
}

impl Cmd {
    pub fn parse(input: &str) -> Result<Cmd> {
        match input.to_uppercase().as_str() {
            "LOG" => Ok(Cmd::Log),
            "STATUS" => Ok(Cmd::Status),
            "HEAD" => Ok(Cmd::Head),
            "ATTACH" => Ok(Cmd::Attach),
            other => Ok(Cmd::Other(other.into())),
        }
    }
}

impl From<Cmd> for String {
    fn from(value: Cmd) -> Self {
        match &value {
            Cmd::Log => "LOG".to_string(),
            Cmd::Status => "STATUS".to_string(),
            Cmd::Head => "HEAD".to_string(),
            Cmd::Attach => "ATTACH".to_string(),
            Cmd::Other(cmd) => cmd.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CmdErrorKind {
    NotFound(String),
    InvalidArguments,
}

impl From<CmdErrorKind> for String {
    fn from(value: CmdErrorKind) -> Self {
        match value {
            CmdErrorKind::InvalidArguments => {
                "ATTACH requires a program argument".into()
            }
            CmdErrorKind::NotFound(program) => {
                format!("{program} is not a Taskmaster Program")
            }
        }
    }
}

pub trait CmdHandler {
    fn handle(&mut self, request: &mut Request) -> result::Result<String, CmdError>;
    fn attach(&mut self, request: &mut Request) -> result::Result<String, CmdError>;
    fn log(&self, request: &mut Request) -> result::Result<String, CmdError>;
    fn head(&self, request: &mut Request) -> result::Result<String, CmdError>;
    fn status(&self, request: &mut Request) -> result::Result<String, CmdError>;
    fn other(&self, request: &mut Request) -> result::Result<String, CmdError>;
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cmd = match self {
            Cmd::Log => "LOG",
            Cmd::Status => "STATUS",
            Cmd::Head => "HEAD",
            Cmd::Attach => "ATTACH",
            Cmd::Other(cmd) => cmd.as_str(),
        };
        write!(f, "{cmd}")
    }
}

#[derive(Debug)]
pub struct CmdError {
    pub message: String,
}

impl std::error::Error for CmdError {}
pub type Result<Cmd> = result::Result<Cmd, CmdError>;

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<&str> for CmdError {
    fn from(msg: &str) -> Self {
        CmdError {
            message: msg.into(),
        }
    }
}

impl From<CmdError> for String {
    fn from(value: CmdError) -> Self { value.message }
}

impl From<CmdErrorKind> for CmdError {
    fn from(value: CmdErrorKind) -> Self {
        CmdError {
            message: value.into(),
        }
    }
}

impl fmt::Display for CmdErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<String> for CmdError {
    fn from(msg: String) -> Self { CmdError { message: msg } }
}

impl CmdError {
    pub fn new(msg: &str) -> CmdError {
        CmdError {
            message: msg.into(),
        }
    }
}

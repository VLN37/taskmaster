use core::fmt;
use std::result;

#[derive(Debug)]
pub enum Cmd {
    Log,
    Status,
    Head,
    Attach,
}

impl Cmd {
    pub fn parse(input: &str) -> Result<Cmd> {
        match input.to_uppercase().as_str() {
            "LOG" => Ok(Cmd::Log),
            "STATUS" => Ok(Cmd::Status),
            "HEAD" => Ok(Cmd::Head),
            "ATTACH" => Ok(Cmd::Attach),
            _ => Err(CmdError::new(&format!("Invalid command: {input}"))),
        }
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

impl CmdError {
    pub fn new(msg: &str) -> CmdError {
        CmdError {
            message: msg.into(),
        }
    }
}

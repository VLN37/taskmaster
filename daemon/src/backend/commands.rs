use core::fmt;
use std::result;

#[derive(Debug)]
pub enum DaemonCommand {
    Log,
    Status,
    Head,
    Attach,
}

pub struct TMCommandError;
type Result<TMCommand> = result::Result<TMCommand, TMCommandError>;

impl fmt::Display for TMCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid command")
    }
}

impl DaemonCommand {
    pub fn parse(input: &str) -> Result<DaemonCommand> {
        match input.to_uppercase().as_str() {
            "LOG" => Ok(DaemonCommand::Log),
            "STATUS" => Ok(DaemonCommand::Status),
            "HEAD" => Ok(DaemonCommand::Head),
            "ATTACH" => Ok(DaemonCommand::Attach),
            _ => Err(TMCommandError),
        }
    }
}

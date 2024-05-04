use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Cmd {
    Log,
    Status,
    Head,
    Attach,
    Other(String),
}

impl Cmd {
    pub fn parse(input: &str) -> super::Result<Cmd> {
        match input.to_uppercase().as_str() {
            "LOG" => Ok(Cmd::Log),
            "STATUS" => Ok(Cmd::Status),
            "HEAD" => Ok(Cmd::Head),
            "ATTACH" => Ok(Cmd::Attach),
            other => Ok(Cmd::Other(other.into())),
        }
    }
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

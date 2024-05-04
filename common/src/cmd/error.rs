use std::fmt;

#[derive(Debug)]
pub struct CmdError {
    pub message: String,
}

impl std::error::Error for CmdError {}

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

impl From<String> for CmdError {
    fn from(msg: String) -> Self { CmdError { message: msg } }
}

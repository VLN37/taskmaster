use core::fmt;
use std::io::{self, RawOsError};

pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Debug)]
pub struct ServerError {
    pub kind:    String,
    pub message: String,
}

impl std::error::Error for ServerError {}
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for ServerError {
    fn from(error: io::Error) -> Self {
        ServerError {
            kind:    String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<RawOsError> for ServerError {
    fn from(error: RawOsError) -> Self {
        ServerError {
            kind:    String::from("OS Error"),
            message: error.to_string(),
        }
    }
}

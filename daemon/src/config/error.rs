use std::{fmt, io};

use common::server::ServerError;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConfigError {
    kind:    String,
    message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError {
            kind:    String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<String> for ConfigError {
    fn from(error: String) -> Self {
        ConfigError {
            kind:    String::from("ConfigError"),
            message: error,
        }
    }
}

impl From<&str> for ConfigError {
    fn from(error: &str) -> Self {
        ConfigError {
            kind:    String::from("ConfigError"),
            message: error.to_string(),
        }
    }
}

impl From<ConfigError> for ServerError {
    fn from(error: ConfigError) -> Self {
        ServerError {
            kind:    error.kind,
            message: error.message,
        }
    }
}

pub use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::config::exceptions::ImproperlyConfigured;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum RestartOption {
    ALWAYS,
    NEVER,
    ONERROR,
}

impl FromStr for RestartOption {
    type Err = ImproperlyConfigured;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ALWAYS" => Ok(RestartOption::ALWAYS),
            "NEVER" => Ok(RestartOption::NEVER),
            "ON_ERROR" => Ok(RestartOption::ONERROR),
            _ => Err(ImproperlyConfigured),
        }
    }
}

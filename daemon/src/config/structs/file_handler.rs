#![allow(clippy::upper_case_acronyms)]

pub use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::config::exceptions::ImproperlyConfigured;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum KnownHandler {
    DEFAULT,
    DISCARD,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum IOHandler {
    KNOWN(KnownHandler),
    FILE(String),
}

impl FromStr for IOHandler {
    type Err = ImproperlyConfigured;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEFAULT" => Ok(IOHandler::KNOWN(KnownHandler::DEFAULT)),
            "DISCARD" => Ok(IOHandler::KNOWN(KnownHandler::DISCARD)),
            other => Ok(IOHandler::FILE(other.into())),
        }
    }
}

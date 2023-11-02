use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, fs::File};
mod structs;
pub use crate::config::structs::{Program, RestartOption, FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskMasterConfig {
    pub programs: HashMap<String, Program>,
}

#[derive(Debug, Clone)]
pub struct ImproperlyConfigured;

impl fmt::Display for ImproperlyConfigured {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid option")
    }
}

pub fn read(file: File) -> TaskMasterConfig {
    serde_yaml::from_reader(file).expect("y u no read.")
}

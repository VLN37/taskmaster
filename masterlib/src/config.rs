mod structs;

use std::{collections::HashMap, fmt, fs::File};
use serde::{Deserialize, Serialize};

pub use crate::config::structs::{Program, RestartOption, FromStr};
pub use structs::*;

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

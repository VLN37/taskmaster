pub mod exceptions;
pub mod structs;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};
pub use structs::{Program, RestartOption, Signal};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskMasterConfig {
    pub programs: HashMap<String, Program>,
}

pub fn read(file: File) -> TaskMasterConfig {
    serde_yaml::from_reader(file).expect("y u no read.")
}

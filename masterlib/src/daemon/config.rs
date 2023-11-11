use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

pub mod exceptions;
pub mod structs;
pub use structs::{Program, RestartOption, Signal};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskMasterConfig {
    pub programs: HashMap<String, Program>,
}

pub fn read(file: File) -> Result<TaskMasterConfig, serde_yaml::Error> {
    serde_yaml::from_reader(file)
}

impl From<File> for TaskMasterConfig {
    fn from(file: File) -> Self {
        match serde_yaml::from_reader(file) {
            Ok(config) => config,
            Err(e) => panic!("{:?}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_parse_test() {
        let manifest = String::from(env!("CARGO_MANIFEST_DIR"));
        let remove_crate_name = manifest.find("/masterlib").unwrap();
        let root = format!("{}/{}", &manifest[..remove_crate_name], crate::CONFIG_PATH);
        let f = std::fs::File::open(root).expect("Could not open file.");
        let config = self::read(f);
        assert!(config.is_ok());
    }
}

use std::collections::HashMap;
use std::fs::File;

use serde::{Deserialize, Serialize};

pub mod exceptions;
pub mod structs;
pub use structs::{ProgramConfig, RestartOption, Signal};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskMasterConfig {
    pub programs: HashMap<String, ProgramConfig>,
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConfigError {}

impl TaskMasterConfig {
    pub fn read(file: File) -> Result<TaskMasterConfig, serde_yaml::Error> {
        serde_yaml::from_reader(file)
    }

    pub fn validate(&self) -> Result<(), String> {
        for v in self.programs.values() {
            v.validate()?;
        }
        Ok(())
    }
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
    use logger::debug;

    use super::*;
    use crate::defs;

    #[test]
    fn config_parse_test() {
        let manifest = String::from(env!("CARGO_MANIFEST_DIR"));
        debug!("{manifest}");
        let index = manifest.find("/daemon").unwrap();
        let root = format!("{}/{}", &manifest[..index], defs::DFL_CONFIG_FILE);
        let f = std::fs::File::open(root).expect("Could not open file.");
        let config = self::TaskMasterConfig::read(f);
        assert!(config.is_ok());
    }
}

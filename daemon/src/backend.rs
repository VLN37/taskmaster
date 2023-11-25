use std::collections::HashMap;
use std::fs::File;

use common::server::{Key, Request};
use common::CONFIG_PATH;

use crate::config::{ConfigError, Program};
use crate::TaskMasterConfig;

#[derive(Default)]
pub struct BackEnd {
    pub config:  TaskMasterConfig,
    pub clients: HashMap<Key, Request>,
}

fn read_config_file() -> TaskMasterConfig {
    let config_file = File::open(CONFIG_PATH).expect("Failed to open config file");
    TaskMasterConfig::from(config_file)
}

impl BackEnd {
    pub fn new() -> BackEnd { BackEnd::default() }

    pub fn get_response_for(&self, key: Key) -> String { format!("Response for {key}") }

    pub fn build(&mut self) {
        self.config = read_config_file();
        print_programs("initial programs", &self.config.programs);
    }

    pub fn update(&mut self) -> Result<(), ConfigError> {
        let new_config = read_config_file();

        if self.config != new_config {
            println!("Updating config");
            self.update_state(new_config);
        } else {
            println!("No changes detected.");
        }

        Ok(())
    }

    fn update_state(&mut self, new_config: TaskMasterConfig) {
        print_programs("current programs", &self.config.programs);
        print_programs("new programs", &new_config.programs);

        let programs_to_kill = get_diff(&self.config.programs, &new_config.programs);
        print_programs("programs to kill", &programs_to_kill);
        let programs_to_spawn = get_diff(&new_config.programs, &self.config.programs);
        print_programs("programs to spawn", &programs_to_spawn);

        self.config = new_config;
    }
}

fn print_programs(msg: &str, programs: &HashMap<String, Program>) {
    println!("---- {msg}");
    let mut programs = programs.keys().collect::<Vec<_>>();

    programs.sort();
    programs.iter().for_each(|p| println!("  {p}"));
}

fn get_diff(
    first: &HashMap<String, Program>,
    second: &HashMap<String, Program>,
) -> HashMap<String, Program> {
    first
        .iter()
        .filter(|&(key, _)| !second.contains_key(key))
        .map(|(key, program)| (key.to_owned(), program.clone()))
        .collect()
}

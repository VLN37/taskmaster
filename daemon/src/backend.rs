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

}

fn print_programs(msg: &str, programs: &HashMap<String, Program>) {
    println!("---- {msg}");
    let mut programs = programs.keys().collect::<Vec<_>>();

    programs.sort();
    programs.iter().for_each(|p| println!("  {p}"));
}

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
        .filter(|&(key, program)| {
            !second.contains_key(key) || has_major_changes(program, &second[key])
        })
        .map(|(key, program)| (key.to_owned(), program.clone()))
        .collect()
}

fn has_major_changes(first: &Program, second: &Program) -> bool {
    first.command != second.command
        || first.args != second.args
        // || first.status != second.status
        // || first.processes != second.processes
        // || first.run_at_startup != second.run_at_startup
        // || first.retry_start_count != second.retry_start_count
        // || first.restart != second.restart
        // || first.graceful_exit != second.graceful_exit
        // || first.ttk != second.ttk
        // || first.success_codes != second.success_codes.clone()
        // || first.succesful_start_after != second.succesful_start_after
        || first.workdir != second.workdir
        || first.environment_variables != second.environment_variables
        || first.umask != second.umask
}

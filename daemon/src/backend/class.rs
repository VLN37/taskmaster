use std::collections::HashMap;

use common::request::Request;
use common::server::Key;
use common::{Cmd, CmdHandler, Response};
use logger::{debug, info};

use super::print_functions::{print_processes, print_programs};
use super::{Client, ClientState, Process, Program};
use crate::config::{ConfigError, ProgramConfig};
use crate::TaskMasterConfig;

#[derive(Default)]
pub struct BackEnd {
    pub config:   TaskMasterConfig,
    pub programs: HashMap<String, Program>,
    pub clients:  HashMap<Key, Client>,
}

impl BackEnd {
    pub fn new(config: TaskMasterConfig) -> BackEnd {
        BackEnd {
            config,
            ..Default::default()
        }
    }

    pub fn get_response_for(&mut self, key: Key) -> Option<Response> {
        let client = self.clients.get_mut(&key).unwrap();
        let mut request = match client.requests.front_mut() {
            Some(request) => request.clone(),
            None => return None,
        };

        let msg = self.handle(&mut request).unwrap_or_else(|x| x.into());
        if request.finished {
            let client = self.clients.get_mut(&key).unwrap();
            client.requests.pop_front();
        }
        let response = Response {
            message:  msg,
            finished: request.finished,
        };
        Some(response)
    }

    pub fn is_finished(&mut self, key: Key) -> bool {
        let client = self.clients.get(&key).unwrap();
        client.requests.front().unwrap().finished
    }

    pub fn start(&mut self) {
        print_programs("initial programs", &self.config.programs);
        self.programs = Self::create_programs(&self.config.programs);

        Self::start_procesess(&mut self.programs);

        print_processes(&self.programs);
    }

    pub fn update_processes_status(&mut self) {
        self.programs
            .iter_mut()
            .for_each(|(_, program)| program.update_process_status())
    }

    pub fn handle_dead_processes(&mut self) {
        self.programs.iter_mut().for_each(|(_, program)| {
            program.update_process_status();
        });

        self.dump_processes_status();
    }

    pub fn insert(&mut self, key: Key, mut request: Request) {
        if let Some(client) = self.clients.get_mut(&key) {
            if client.state != ClientState::Unattached {
                request.command = Cmd::Other(request.command.into());
            }
            client.requests.push_back(request);
        } else {
            let mut client = Client::new();
            client.requests.push_back(request);
            self.clients.insert(key, client);
        }
    }

    pub fn is_attached(&self, k: Key) -> bool {
        let c = self.clients.get(&k).expect("the client to exist");
        c.state != ClientState::Unattached
    }

    fn start_procesess(programs: &mut HashMap<String, Program>) {
        programs.iter_mut().for_each(|(_, program)| {
            program.processes = (0..program.config.processes)
                .map(|_| Process::start(&mut program.command))
                .collect();

            program.update_process_status();
        });
    }

    fn create_programs(
        program_configs: &HashMap<String, ProgramConfig>,
    ) -> HashMap<String, Program> {
        program_configs
            .iter()
            .map(|config_pair| {
                (config_pair.0.to_owned(), Program::build_from(config_pair))
            })
            .collect()
    }

    pub fn update(&mut self, new_config: TaskMasterConfig) -> Result<(), ConfigError> {
        if self.config != new_config {
            info!("Updating config");
            self.update_state(new_config);
        } else {
            info!("No changes detected.");
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

    pub fn dump_processes_status(&self) {
        debug!("{}", print_processes(&self.programs));
    }
}

fn get_diff(
    first_list: &HashMap<String, ProgramConfig>,
    second_list: &HashMap<String, ProgramConfig>,
) -> HashMap<String, ProgramConfig> {
    first_list
        .iter()
        .filter(|&(key_in_first, config)| {
            !second_list.contains_key(key_in_first)
                || has_major_changes(config, &second_list[key_in_first])
        })
        .map(|(key, program)| (key.to_owned(), program.clone()))
        .collect()
}

fn has_major_changes(first: &ProgramConfig, second: &ProgramConfig) -> bool {
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
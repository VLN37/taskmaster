use std::collections::HashMap;

use common::{CmdHandler, Request, Response};
use logger::{debug, info};

use super::print_functions::{print_processes, print_programs};
use super::{Process, Program};
use crate::config::{ConfigError, ProgramConfig};
use crate::TaskMasterConfig;

#[derive(Default)]
pub struct BackEnd {
    pub config:   TaskMasterConfig,
    pub programs: HashMap<String, Program>,
}

impl BackEnd {
    pub fn new(config: TaskMasterConfig) -> Self {
        BackEnd {
            config,
            ..Default::default()
        }
    }

    pub fn handle_request(&mut self, request: &mut Request) -> Response {
        let msg = self.handle(request).unwrap_or_else(|x| x.into());
        Response {
            message:  msg,
            finished: request.finished,
        }
    }

    pub fn start(&mut self) {
        print_programs("initial programs", &self.config.programs);
        self.programs = Self::create_programs(&self.config.programs);

        self.create_startup_processes();

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

    fn create_startup_processes(&mut self) {
        self.programs
            .iter_mut()
            .filter(|(_, program)| program.config.run_at_startup)
            .for_each(|(_, program)| {
                program.processes =
                    Self::create_processes(program, program.config.processes);

                program.update_process_status();
            });
    }

    fn create_processes(program: &mut Program, count: usize) -> Vec<Process> {
        (0..count)
            .map(|_| Process::start(&mut program.command))
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

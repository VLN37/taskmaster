use std::collections::HashMap;
use std::io::{self, Error};
use std::process::{Child, Command};

use common::server::{Key, Request};
use logger::info;

mod print_functions;
use self::print_functions::{print_processes, print_programs};
use crate::config::{ConfigError, ProgramConfig};
use crate::TaskMasterConfig;

#[derive(Default)]
pub struct BackEnd {
    pub config:   TaskMasterConfig,
    pub programs: HashMap<String, Program>,
    pub clients:  HashMap<Key, Request>,
}

pub struct Program {
    pub config_name: String,
    pub config:      ProgramConfig,
    pub command:     Command,
    pub processes:   Vec<Process>,
}

pub struct Process {
    child:  Result<Child, Error>,
    status: ProcessStatus,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProcessStatus {
    Starting,
    FailedToStart,
    Active,
    GracefulExit,
    Killed,
    FailedExit,
    Unknown,
}

impl BackEnd {
    pub fn new(config: TaskMasterConfig) -> BackEnd {
        BackEnd {
            config,
            ..Default::default()
        }
    }

    pub fn get_response_for(&self, key: Key) -> String { format!("Response for {key}") }

    pub fn start(&mut self) {
        print_programs("initial programs", &self.config.programs);
        self.programs = Self::create_programs(&self.config.programs);

        Self::start_procesess(&mut self.programs);

        print_processes(&self.programs);
    }

    fn start_process(program: &mut Program) -> Process {
        if program.command.get_program() == "" {
            return Process {
                child:  Err(Error::new(io::ErrorKind::Other, "Empty command")),
                status: ProcessStatus::FailedToStart,
            };
        }
        Process {
            child:  program.command.spawn(),
            status: ProcessStatus::Starting,
        }
    }

    fn start_procesess(programs: &mut HashMap<String, Program>) {
        programs.iter_mut().for_each(|(_name, program)| {
            program.processes = (0..program.config.processes as usize)
                .map(|_| Self::start_process(program))
                .collect();

            program
                .processes
                .iter_mut()
                .for_each(|p| p.update_status(&program.config))
        });
    }

    fn create_programs(
        program_configs: &HashMap<String, ProgramConfig>,
    ) -> HashMap<String, Program> {
        program_configs
            .iter()
            .map(|(program_name, command_config)| {
                let mut command = Command::new(&command_config.command);
                command
                    .current_dir(&command_config.workdir)
                    .args(&command_config.args)
                    .envs(
                        command_config
                            .environment_variables
                            .iter()
                            .filter_map(|var| var.split_once('='))
                            .map(|(var, value)| (var.to_string(), value.to_string()))
                            .collect::<HashMap<String, String>>(),
                    );

                (
                    program_name.to_owned(),
                    Program {
                        config_name: program_name.to_owned(),
                        config: command_config.clone(),
                        command,
                        processes: vec![],
                    },
                )
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
}

impl Process {
    pub fn update_status(&mut self, config: &ProgramConfig) {
        self.status = match &mut self.child {
            Ok(ref mut child) => match child.try_wait() {
                // should check status, could still have crashed or something
                Ok(Some(status)) => {
                    if let Some(code) = status.code() {
                        if config.success_codes.contains(&(code as u32)) {
                            ProcessStatus::GracefulExit
                        } else {
                            ProcessStatus::FailedExit
                        }
                    } else {
                        ProcessStatus::Unknown
                    }
                }
                Ok(None) => ProcessStatus::Active,
                Err(_) => ProcessStatus::FailedExit,
            },
            Err(_) => ProcessStatus::FailedToStart,
        };
    }
}

fn get_diff(
    first: &HashMap<String, ProgramConfig>,
    second: &HashMap<String, ProgramConfig>,
) -> HashMap<String, ProgramConfig> {
    first
        .iter()
        .filter(|&(key, program)| {
            !second.contains_key(key) || has_major_changes(program, &second[key])
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

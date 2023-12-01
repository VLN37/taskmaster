use std::collections::HashMap;
use std::io::{self, Error};
use std::process::{Child, Command};

use common::server::{Key, Request};
use logger::{debug, info};

use crate::config::{ConfigError, ProgramConfig};
use crate::TaskMasterConfig;

#[derive(Default)]
pub struct BackEnd {
    pub config:   TaskMasterConfig,
    pub programs: HashMap<String, ActiveProgram>,
    pub clients:  HashMap<Key, Request>,
}

pub struct ActiveProgram {
    pub config_name: String,
    pub config:      ProgramConfig,
    pub command:     Command,
    pub processes:   Vec<Result<Child, Error>>,
    pub status:      Vec<ProgramStatus>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProgramStatus {
    Starting,
    FailedToStart,
    Active,
    GracefulExit,
    Killed,
    FailedExit,
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

        self.programs.iter().for_each(|(_, program)| {
            program
                .processes
                .iter()
                .enumerate()
                .for_each(|(i, process)| {
                    let pid_or_error = match process {
                        Ok(child) => child.id().to_string(),
                        Err(err) => err.to_string(),
                    };
                    info!(
                        "{}[{}]: {:?} [{}]",
                        program.config_name, i, program.status[i], pid_or_error
                    );
                })
        })
    }

    fn start_process(program: &mut ActiveProgram) -> Result<Child, Error> {
        if program.command.get_program() == "" {
            return Err(Error::new(io::ErrorKind::Other, "Empty command"));
        }

        program.command.spawn()
    }

    fn start_procesess(programs: &mut HashMap<String, ActiveProgram>) {
        programs.iter_mut().for_each(|(_name, program)| {
            program.processes = (0..program.config.processes as usize)
                .map(|index| {
                    let result = Self::start_process(program);
                    update_process_status(program, index, &result);
                    result
                })
                .collect();
        });
    }

    fn create_programs(
        program_configs: &HashMap<String, ProgramConfig>,
    ) -> HashMap<String, ActiveProgram> {
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

                let status =
                    vec![ProgramStatus::Starting; command_config.processes as usize];

                (
                    program_name.to_owned(),
                    ActiveProgram {
                        config_name: program_name.to_owned(),
                        config: command_config.clone(),
                        command,
                        processes: vec![],
                        status,
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

fn update_process_status(
    program: &mut ActiveProgram,
    index: usize,
    spawn_result: &Result<Child, Error>,
) {
    program.status[index] = match *spawn_result {
        Ok(_) => ProgramStatus::Active,
        Err(_) => ProgramStatus::FailedToStart,
    };
}

fn print_programs(msg: &str, programs: &HashMap<String, ProgramConfig>) {
    debug!("---- {msg}");
    let mut programs = programs.keys().collect::<Vec<_>>();

    programs.sort();
    programs.iter().for_each(|p| debug!("  {p}"));
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

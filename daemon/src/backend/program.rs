use std::cmp::Ordering;
use std::collections::HashMap;
use std::process::Command;

use super::process::Process;
use crate::config::ProgramConfig;

pub struct Program {
    pub config_name: String,
    pub config:      ProgramConfig,
    pub command:     Command,
    pub processes:   Vec<Process>,
}

impl Program {
    pub fn build_from(
        (config_name, command_config): (&String, &ProgramConfig),
    ) -> Program {
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
        Program {
            config_name: config_name.to_string(),
            config: command_config.clone(),
            command,
            processes: vec![],
        }
    }

    pub fn update_process_status(&mut self) {
        self.processes.iter_mut().for_each(|p| {
            p.update_status(&self.config);
            if p.should_restart {
                p.restart(&mut self.command);
            } else if p.should_try_again {
                p.try_start_again(&mut self.command);
            }
        })
    }

    pub fn update_process_count(&mut self) {
        let current_count = self.processes.len();
        let desired_count = self.config.processes;
        match current_count.cmp(&desired_count) {
            Ordering::Less => {
                for _ in 0..desired_count - current_count {
                    self.processes.push(Process::start(&mut self.command));
                }
            }
            Ordering::Greater => {
                for _ in 0..current_count - desired_count {
                    self.processes.pop();
                }
            }
            _ => {}
        }
    }
}

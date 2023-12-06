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
        self.processes
            .iter_mut()
            .for_each(|p| p.update_status(&self.config))
    }
}

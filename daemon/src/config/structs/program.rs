use serde::{Deserialize, Serialize};

use super::{RestartOption, Signal};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ProgramConfig {
    pub command:               String,
    pub args:                  Vec<String>,
    pub processes:             u32,
    pub run_at_startup:        bool,
    pub retry_start_count:     u32,
    pub restart:               RestartOption,
    pub graceful_exit:         Signal,
    pub ttk:                   u32,
    pub success_codes:         Vec<u32>,
    pub succesful_start_after: u32, // seconds
    pub workdir:               String,
    pub environment_variables: Vec<String>,
    pub umask:                 u32,
}

impl ProgramConfig {
    pub fn new() -> ProgramConfig { ProgramConfig::default() }
}

impl Default for ProgramConfig {
    fn default() -> ProgramConfig {
        ProgramConfig {
            command:               String::default(),
            args:                  vec![],
            processes:             1,
            run_at_startup:        true,
            retry_start_count:     3,
            restart:               RestartOption::ONERROR,
            graceful_exit:         Signal::SIGQUIT,
            ttk:                   10,
            success_codes:         vec![0],
            succesful_start_after: 0,
            workdir:               std::env::var("CWD").unwrap_or(String::from("/")),
            environment_variables: vec![],
            umask:                 420,
        }
    }
}

// impl Copy for Program {}

impl Clone for ProgramConfig {
    fn clone(&self) -> Self {
        Self {
            command:               self.command.clone(),
            args:                  self.args.clone(),
            processes:             self.processes,
            run_at_startup:        self.run_at_startup,
            retry_start_count:     self.retry_start_count,
            restart:               self.restart,
            graceful_exit:         self.graceful_exit,
            ttk:                   self.ttk,
            success_codes:         self.success_codes.clone(),
            succesful_start_after: self.succesful_start_after,
            workdir:               self.workdir.clone(),
            environment_variables: self.environment_variables.clone(),
            umask:                 self.umask,
        }
    }
}

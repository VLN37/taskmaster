use serde::{Deserialize, Serialize};

use super::{ProgramStatus, RestartOption, Signal};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Program {
    pub command:               String,
    pub args:                  String,
    pub status:                ProgramStatus,
    pub processes:             u32,
    pub run_at_startup:        bool,
    pub retry_start_count:     u32,
    pub restart:               RestartOption,
    pub graceful_exit:         Signal,
    pub ttk:                   u32,
    pub success_codes:         Vec<u32>,
    pub succesful_start_after: u32, // seconds
    pub workdir:               String,
    pub environment_variables: String,
    pub umask:                 u32,
}

impl Program {
    pub fn new() -> Program { Program::default() }
}

impl Default for Program {
    fn default() -> Program {
        Program {
            command:               String::from("tail -f"),
            args:                  String::from(""),
            status:                ProgramStatus::Starting,
            processes:             1,
            run_at_startup:        true,
            retry_start_count:     3,
            restart:               RestartOption::ALWAYS,
            graceful_exit:         Signal::SIGQUIT,
            ttk:                   10,
            success_codes:         [0].to_vec(),
            succesful_start_after: 5,
            workdir:               std::env::var("CWD").unwrap_or(String::from("/")),
            environment_variables: String::from("ANSWER=42"),
            umask:                 420,
        }
    }
}

// impl Copy for Program {}

impl Clone for Program {
    fn clone(&self) -> Self {
        Self {
            command:               self.command.clone(),
            args:                  self.args.clone(),
            status:                self.status,
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

use serde::{Deserialize, Serialize};

use super::ProgramStatus;
use super::RestartOption;
use super::Signal;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Program {
    pub command: String,
    pub args: String,
    pub status: ProgramStatus,
    pub processes: u32,
    pub run_at_startup: bool,
    pub retry_start_count: u32,
    pub restart: RestartOption,
    pub graceful_exit: Signal,
    pub ttk: u32,
    pub success_codes: Vec<u32>,
    pub succesful_start_after: u32, // seconds
    pub workdir: String,
    pub environment_variables: String,
    pub umask: u32,
}

impl Default for Program {
    fn default() -> Program {
        Program {
            command: String::from("tail -f"),
            args: String::from(""),
            status: ProgramStatus::Starting,
            processes: 1,
            run_at_startup: true,
            retry_start_count: 3,
            restart: RestartOption::ALWAYS,
            graceful_exit: Signal::SIGQUIT,
            ttk: 10,
            success_codes: [0].to_vec(),
            succesful_start_after: 5,
            workdir: std::env::var("PWD").unwrap(),
            environment_variables: String::from("ANSWER=42"),
            umask: 420,
        }
    }
}

use std::io::{self, Error};
use std::process::Child;

use crate::backend::program::Program;
use crate::config::ProgramConfig;

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

impl std::fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{:?}", self))
    }
}

pub struct Process {
    pub child:     Result<Child, Error>,
    pub exit_code: u32,
    pub status:    ProcessStatus,
}

impl std::fmt::Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pid_or_error = match &self.child {
            Ok(child) => child.id().to_string(),
            Err(err) => err.to_string(),
        };
        write!(f, "{:15} {}", self.status, pid_or_error)
    }
}

impl Process {
    pub fn start(program: &mut Program) -> Process {
        if program.command.get_program() == "" {
            return Process {
                child:     Err(Error::new(io::ErrorKind::Other, "Empty command")),
                exit_code: 0,
                status:    ProcessStatus::FailedToStart,
            };
        }
        Process {
            child:     program.command.spawn(),
            exit_code: 0,
            status:    ProcessStatus::Starting,
        }
    }

    pub fn update_status(&mut self, config: &ProgramConfig) {
        self.status = match &mut self.child {
            Ok(ref mut child) => match child.try_wait() {
                // should check status, could still have crashed or something
                Ok(Some(status)) => {
                    if let Some(code) = status.code() {
                        self.exit_code = code as u32;
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

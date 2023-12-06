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

pub struct Process {
    pub child:  Result<Child, Error>,
    pub status: ProcessStatus,
}

impl Process {
    pub fn start(program: &mut Program) -> Process {
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

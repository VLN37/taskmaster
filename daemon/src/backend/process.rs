use std::io::{self, Error};
use std::os::unix::process::ExitStatusExt;
use std::process::{Child, Command};
#[cfg(not(test))]
use std::time::{Duration, Instant};

use logger::{error, info};

#[cfg(test)]
mod tests;
#[cfg(test)]
use self::tests::{Duration, Instant};
use crate::config::{ProgramConfig, RestartOption, Signal};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProcessStatus {
    Starting,
    FailedToStart,
    Active,
    GracefulExit(u32),
    Killed(Signal),
    FailedExit(u32),
}

pub struct Process {
    pub child:            Result<Child, Error>,
    pub status:           ProcessStatus,
    pub try_count:        u32,
    pub started_at:       Option<Instant>,
    pub should_try_again: bool,
    pub should_restart:   bool,
}

impl Process {
    pub fn new(
        child_result: Result<Child, Error>,
        initial_status: ProcessStatus,
    ) -> Process {
        let started_at = child_result.is_ok().then_some(Instant::now()).or(None);

        Process {
            child: child_result,
            status: initial_status,
            started_at,
            ..Process::default()
        }
    }

    fn spawn_process(command: &mut Command) -> Result<Child, Error> {
        if command.get_program() == "" {
            return Err(Error::new(io::ErrorKind::Other, "Empty command"));
        }

        command.spawn()
    }

    pub fn start(command: &mut Command) -> Process {
        let child = Process::spawn_process(command);

        let status = if child
            .as_ref()
            .is_err_and(|err| err.to_string() == "Empty command")
        {
            ProcessStatus::FailedToStart
        } else {
            ProcessStatus::Starting
        };

        Process::new(child, status)
    }

    pub fn update_status(&mut self, config: &ProgramConfig) {
        if self.child.is_err() {
            self.status = ProcessStatus::FailedToStart;
            return;
        }

        match self.status {
            ProcessStatus::Starting => self.handle_starting_phase(config),
            ProcessStatus::FailedToStart => {}
            ProcessStatus::Active => self.handle_active_phase(config),
            ProcessStatus::GracefulExit(_) => self.handle_graceful_exit_phase(config),
            ProcessStatus::Killed(_) => self.handle_killed_phase(config),
            ProcessStatus::FailedExit(_) => self.handle_failed_exit_phase(config),
        }

        // self.update_status_match(config);
    }

    pub fn restart(&mut self, command: &mut Command) {
        self.should_restart = false;
        if self.status == ProcessStatus::FailedToStart {
            return;
        }

        info!("Restarting process {:?}", command.get_program());
        self.child = Process::spawn_process(command);
        self.status = ProcessStatus::Starting;
    }

    pub fn try_start_again(&mut self, command: &mut Command) {
        self.should_try_again = false;
        if self.status == ProcessStatus::FailedToStart {
            return;
        }
        self.try_count += 1;
        info!(
            "Trying to start process again [retry {}] {:?}",
            self.try_count,
            command.get_program(),
        );

        self.child = Process::spawn_process(command);
        self.started_at = Some(Instant::now());
    }

    fn handle_starting_phase(&mut self, config: &ProgramConfig) {
        if self.try_count >= config.retry_start_count {
            self.status = ProcessStatus::FailedToStart;
        } else {
            let time_elapsed = self.time_elapsed();

            let expected_duration =
                Duration::from_secs(config.succesful_start_after as u64);

            let child = self.child.as_mut().unwrap();
            if is_alive(child) {
                if time_elapsed > expected_duration {
                    self.status = ProcessStatus::Active;
                }
            } else {
                match child.wait() {
                    Ok(status) => {
                        if time_elapsed >= expected_duration {
                            if let Some(code) = status.code() {
                                if config.success_codes.contains(&(code as u32)) {
                                    self.status =
                                        ProcessStatus::GracefulExit(code as u32);
                                    self.handle_graceful_exit_phase(config);
                                } else {
                                    self.status =
                                        ProcessStatus::FailedExit(code as u32);
                                    self.handle_failed_exit_phase(config);
                                }
                            } else if let Some(signal) = status.signal() {
                                self.status =
                                    ProcessStatus::Killed(Signal::from(signal));
                                self.handle_killed_phase(config)
                            } else {
                                error!(
                                    "Error evaluating process status: : {}",
                                    config.command
                                );
                            }
                        } else {
                            self.should_try_again = true;
                        }
                    }
                    Err(_) => todo!("How could the wait fail?"),
                }
            }
        }
    }

    fn handle_active_phase(&mut self, config: &ProgramConfig) {
        let child = self
            .child
            .as_mut()
            .expect("Process was active, but there was no child!!");
        match child.try_wait() {
            Ok(None) => {} // it's still alive
            Ok(Some(status)) => {
                if let Some(code) = status.code() {
                    if config.success_codes.contains(&(code as u32)) {
                        self.status = ProcessStatus::GracefulExit(code as u32);
                    } else {
                        self.status = ProcessStatus::FailedExit(code as u32);
                    }
                } else {
                    let signal = status.signal().unwrap();
                    self.status = ProcessStatus::Killed(Signal::from(signal));
                }
            }
            Err(err) => {
                error!(
                    "Error attempting to wait for child [{}]: {}",
                    config.command, err
                );
            }
        }
    }

    fn handle_graceful_exit_phase(&mut self, config: &ProgramConfig) {
        match config.restart {
            RestartOption::ALWAYS => {
                self.should_restart = true;
            }
            RestartOption::ONERROR => {}
            RestartOption::NEVER => {}
        }
    }

    fn handle_killed_phase(&mut self, config: &ProgramConfig) {
        match config.restart {
            RestartOption::ALWAYS | RestartOption::ONERROR => {
                self.should_restart = true;
            }
            RestartOption::NEVER => {}
        }
    }

    fn handle_failed_exit_phase(&mut self, config: &ProgramConfig) {
        match config.restart {
            RestartOption::ALWAYS | RestartOption::ONERROR => {
                self.should_restart = true;
            }
            RestartOption::NEVER => {}
        }
    }

    fn time_elapsed(&mut self) -> Duration {
        self.started_at.get_or_insert(Instant::now()).elapsed()
    }
}

impl std::fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{:?}", self))
    }
}

impl Default for Process {
    fn default() -> Self {
        Process {
            child:            Err(Error::new(
                io::ErrorKind::Other,
                "Unititialized process",
            )),
            status:           ProcessStatus::FailedToStart,
            try_count:        0,
            started_at:       None,
            should_restart:   false,
            should_try_again: false,
        }
    }
}

impl std::fmt::Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pid_or_error = match &self.child {
            Ok(child) => child.id().to_string(),
            Err(err) => format!("Error: {err}"),
        };

        let exit_code = match self.status {
            ProcessStatus::GracefulExit(code) | ProcessStatus::FailedExit(code) => {
                format!("Exit code: {}", code)
            }
            ProcessStatus::Killed(signal) => format!("Signaled: {:?}", signal),
            _ => "".to_string(),
        };

        write!(f, "{:15} {:15} {}", self.status, exit_code, pid_or_error)
    }
}

fn is_alive(child: &mut Child) -> bool { matches!(child.try_wait(), Ok(None)) }

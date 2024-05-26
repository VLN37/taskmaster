use std::collections::HashMap;
use std::io::Error;
use std::os::unix::process::ExitStatusExt;
use std::process::{Child, Command};
#[cfg(not(test))]
use std::time::{Duration, Instant};

use logger::{error, info};

use super::logging::get_logfile;
#[cfg(test)]
use super::tests::{Duration, Instant};
use super::ProcessStatus;
use crate::config::{ProgramConfig, RestartOption, Signal};

pub struct Process {
    pub child:            Result<Child, Error>,
    pub command:          Command,
    pub status:           ProcessStatus,
    pub attempt:          u32,
    pub started_at:       Option<Instant>,
    pub should_try_again: bool,
    pub should_restart:   bool,
    pub stdout_file:      String,
    pub stderr_file:      String,
}

impl Process {
    pub fn new(program_config: &ProgramConfig) -> Process {
        Process {
            child:            Err(Error::other("Unititialized process")),
            command:          Process::make_command(program_config),
            status:           ProcessStatus::Stopped,
            attempt:          0,
            started_at:       None,
            should_try_again: false,
            should_restart:   false,
            stdout_file:      "".to_string(),
            stderr_file:      "".to_string(),
        }
    }

    fn make_command(command_config: &ProgramConfig) -> Command {
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
        command
    }

    fn spawn_process(&mut self) -> Result<Child, Error> {
        if self.command.get_program() == "" {
            return Err(Error::other("Empty command"));
        }

        let stdout_filename = format!("{}-stdout.log", self.stdout_file);
        let stderr_filename = format!("{}-stderr.log", self.stderr_file);
        let stdout_logfile = get_logfile(&stdout_filename)?;
        let stderr_logfile = get_logfile(&stderr_filename)?;

        self.command
            .stdout(stdout_logfile)
            .stderr(stderr_logfile)
            .spawn()
    }

    pub fn start(&mut self) {
        let child = self.spawn_process();

        let status = if child
            .as_ref()
            .is_err_and(|err| err.to_string() == "Empty command")
        {
            ProcessStatus::FailedToStart
        } else {
            self.started_at = Some(Instant::now());
            ProcessStatus::Starting
        };

        self.child = child;
        self.status = status;
    }

    pub fn update_status(&mut self, config: &ProgramConfig) {
        if self.child.is_err() {
            self.status = ProcessStatus::FailedToStart;
            return;
        }

        match self.status {
            ProcessStatus::Stopped => {}
            ProcessStatus::Starting => self.handle_starting_phase(config),
            ProcessStatus::FailedToStart => {}
            ProcessStatus::Active => self.handle_active_phase(config),
            ProcessStatus::GracefulExit(_) => self.handle_graceful_exit_phase(config),
            ProcessStatus::Killed(_) => self.handle_killed_phase(config),
            ProcessStatus::FailedExit(_) => self.handle_failed_exit_phase(config),
        }
    }

    pub fn restart(&mut self) {
        self.should_restart = false;
        if self.status == ProcessStatus::FailedToStart {
            return;
        }

        info!("Restarting process {:?}", self.command.get_program());
        self.child = self.spawn_process();
        self.status = ProcessStatus::Starting;
    }

    pub fn try_start_again(&mut self) {
        self.should_try_again = false;
        if self.status == ProcessStatus::FailedToStart {
            return;
        }
        self.attempt += 1;
        info!(
            "Trying to start process again [retry {}] {:?}",
            self.attempt,
            self.command.get_program(),
        );

        self.child = self.spawn_process();
        self.started_at = Some(Instant::now());
    }

    fn handle_starting_phase(&mut self, config: &ProgramConfig) {
        if self.attempt >= config.retry_start_count {
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
                    Err(e) => todo!("failed waiting for process to finish {e}"),
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

impl Default for Process {
    fn default() -> Self {
        Process {
            child:            Err(Error::other("Unititialized process")),
            command:          Command::new(""),
            status:           ProcessStatus::Stopped,
            attempt:          0,
            started_at:       None,
            should_restart:   false,
            should_try_again: false,
            stdout_file:      "".to_string(),
            stderr_file:      "".to_string(),
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

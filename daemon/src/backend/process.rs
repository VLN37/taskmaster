use std::io::{self, Error};
use std::os::unix::process::ExitStatusExt;
use std::process::{Child, Command};
#[cfg(not(test))]
use std::time::{Duration, Instant};

use logger::{error, info};

use crate::config::{ProgramConfig, RestartOption, Signal};

#[cfg(test)]
mod time_stub;
#[cfg(test)]
use self::time_stub::{Duration, Instant};

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
    pub child:          Result<Child, Error>,
    pub status:         ProcessStatus,
    pub try_count:      u32,
    pub started_at:     Option<Instant>,
    pub should_restart: bool,
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

    pub fn is_dead(&self) -> bool {
        matches!(
            self.status,
            ProcessStatus::FailedToStart
                | ProcessStatus::GracefulExit(_)
                | ProcessStatus::Killed(_)
                | ProcessStatus::FailedExit(_)
        )
    }

    pub fn restart(&mut self, command: &mut Command) {
        info!("Restarting process... {:?}", command.get_program());
        if self.status == ProcessStatus::FailedToStart {
            return;
        }

        self.child = Process::spawn_process(command);
        self.try_count += 1;
    }

    fn handle_starting_phase(&mut self, config: &ProgramConfig) {
        if self.try_count > config.retry_start_count {
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
                                } else {
                                    self.status =
                                        ProcessStatus::FailedExit(code as u32);
                                }
                            } else if let Some(signal) = status.signal() {
                                self.status =
                                    ProcessStatus::Killed(Signal::from(signal));
                            } else {
                                error!(
                                    "Error evaluating process status: : {}",
                                    config.command
                                );
                            }
                        } else {
                            todo!("request restart");
                        }
                    }
                    Err(_) => todo!("request restart"),
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
                info!("Restarting process... {}", config.command);
                dbg!(config);
            }
            RestartOption::ONERROR => {}
            RestartOption::NEVER => {}
        }
    }

    fn handle_killed_phase(&mut self, config: &ProgramConfig) {
        match config.restart {
            RestartOption::ALWAYS => todo!(),
            RestartOption::ONERROR => todo!(),
            RestartOption::NEVER => {}
        }
    }

    fn handle_failed_exit_phase(&mut self, config: &ProgramConfig) {
        info!("Process failed: {}", config.command);
        match config.restart {
            RestartOption::ALWAYS | RestartOption::ONERROR => {
                info!("Restarting process... {}", config.command)
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
            child:          Err(Error::new(
                io::ErrorKind::Other,
                "Unititialized process",
            )),
            status:         ProcessStatus::FailedToStart,
            try_count:      0,
            started_at:     None,
            should_restart: false,
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

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::time_stub;
    use crate::backend::process::{Process, ProcessStatus};
    use crate::backend::program::Program;
    use crate::config::{ProgramConfig, RestartOption};

    #[test]
    fn test_process_should_spawn_program_immediately() {
        // given
        let config_name = String::from("test");
        let mut config = ProgramConfig::new();
        config.command = String::from("sleep");
        config.args.push(String::from("1"));
        config.restart = RestartOption::NEVER;
        let mut program = Program::build_from((&config_name, &config));
        // when
        let process = Process::start(&mut program.command);
        // then
        assert!(process.child.is_ok());
    }

    #[test]
    fn test_initial_status_should_be_starting() {
        // given
        let config_name = String::from("test");
        let mut config = ProgramConfig::new();
        config.command = String::from("sleep");
        config.args.push(String::from("1"));
        config.restart = RestartOption::NEVER;
        let mut program = Program::build_from((&config_name, &config));
        // when
        let process = Process::start(&mut program.command);
        // then
        assert_eq!(process.status, ProcessStatus::Starting);
    }

    #[test]
    fn test_status_should_be_gracefulexit_when_program_exits_with_a_status_specified_in_config(
    ) {
        // given
        let config_name = String::from("test");
        let mut config = ProgramConfig::new();

        config.command = String::from("echo");
        config.args.push(String::from("-n"));
        config.restart = RestartOption::NEVER;
        config.success_codes = vec![0];

        let mut program = Program::build_from((&config_name, &config));

        // when
        let mut process = Process::start(&mut program.command);

        // then
        // start as starting
        assert_eq!(process.status, ProcessStatus::Starting);

        process.child.as_mut().unwrap().wait().unwrap();

        process.update_status(&config);

        // then change to graceful exit
        assert_eq!(process.status, ProcessStatus::GracefulExit(0));
    }

    #[test]
    fn test_status_should_be_failed_exit_when_program_exits_with_a_status_not_specified_in_config(
    ) {
        // given
        let config_name = String::from("test");
        let mut config = ProgramConfig::new();
        config.command = String::from("bash");
        config.args.push(String::from("-c"));
        config.args.push(String::from("exit 1"));
        config.restart = RestartOption::NEVER;
        config.success_codes = vec![0];
        let mut program = Program::build_from((&config_name, &config));
        // when
        let mut process = Process::start(&mut program.command);
        // then
        assert_eq!(process.status, ProcessStatus::Starting);
        process.child.as_mut().unwrap().wait().unwrap();
        process.update_status(&config);
        // then change to failed
        assert_eq!(process.status, ProcessStatus::FailedExit(1));
    }

    #[test]
    fn test_status_should_be_killed_when_program_is_killed() {
        // given
        let config_name = String::from("test");
        let mut config = ProgramConfig::new();
        config.command = String::from("sleep");
        config.args.push(String::from("10"));
        config.restart = RestartOption::NEVER;
        let mut program = Program::build_from((&config_name, &config));
        // when
        let mut process = Process::start(&mut program.command);
        // then
        assert_eq!(process.status, ProcessStatus::Starting);
        process.child.as_mut().unwrap().kill().unwrap();
        process.child.as_mut().unwrap().wait().unwrap();
        process.update_status(&config);
        // then change to killed
        assert_eq!(
            process.status,
            ProcessStatus::Killed(crate::config::Signal::SIGKILL)
        );
    }

    fn send_signal(pid: u32, signal: i32) {
        let signal = (-signal).to_string();
        let mut kill = Command::new("kill")
            .args([signal, pid.to_string()])
            .spawn()
            .expect("Failed to spawn kill process");

        kill.wait().expect("Failed to wait for kill process");
    }

    #[test]
    fn test_kill_enum_should_be_equivalent_to_the_signal_sent() {
        // given
        let config_name = String::from("test");
        let mut config = ProgramConfig::new();
        config.command = String::from("sleep");
        config.args.push(String::from("10"));
        config.restart = RestartOption::NEVER;
        let mut program = Program::build_from((&config_name, &config));
        // when
        let mut process = Process::start(&mut program.command);
        // then
        assert_eq!(process.status, ProcessStatus::Starting);
        send_signal(
            process.child.as_mut().unwrap().id(),
            crate::config::Signal::SIGINT as i32,
        );

        process.child.as_mut().unwrap().wait().unwrap();
        process.update_status(&config);
        // then change to killed
        assert_eq!(
            process.status,
            ProcessStatus::Killed(crate::config::Signal::SIGINT)
        );
    }

    #[test]
    fn test_status_should_be_starting_if_not_enough_time_has_passed_since_start() {
        // given
        let config_name = String::from("test");
        let mut config = ProgramConfig::new();

        config.command = String::from("sleep");
        config.args.push(String::from("10"));
        config.restart = RestartOption::NEVER;
        config.succesful_start_after = 5;

        let mut program = Program::build_from((&config_name, &config));
        let mut process = Process::start(&mut program.command);
        assert_eq!(process.status, ProcessStatus::Starting);

        time_stub::Instant::advance(2);

        // when
        process.update_status(&config);

        // then
        assert_eq!(process.status, ProcessStatus::Starting);
    }

    #[test]
    fn test_status_should_be_active_if_enough_time_has_passed_since_start() {
        // given
        let config_name = String::from("test");
        let mut config = ProgramConfig::new();
        config.command = String::from("sleep");
        config.args.push(String::from("10"));
        config.restart = RestartOption::NEVER;
        config.succesful_start_after = 5;
        let mut program = Program::build_from((&config_name, &config));
        let mut process = Process::start(&mut program.command);
        assert_eq!(process.status, ProcessStatus::Starting);
        time_stub::Instant::advance(6);
        // when
        process.update_status(&config);
        // then
        assert_eq!(process.status, ProcessStatus::Active);
    }
}

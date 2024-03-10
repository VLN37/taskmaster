use std::process::Command;

// use super::time_stub;
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
    assert_eq!(process.status, ProcessStatus::Killed(crate::config::Signal::SIGINT));
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

    Instant::advance(2);

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
    Instant::advance(6);
    // when
    process.update_status(&config);
    // then
    assert_eq!(process.status, ProcessStatus::Active);
}

#[test]
fn process_should_restart_if_it_exits_with_an_error_code_and_restart_option_is_always()
{
    // given
    let config_name = String::from("test");
    let mut config = ProgramConfig::new();
    config.command = String::from("bash");
    config.args.push(String::from("-c"));
    config.args.push(String::from("exit 1"));
    config.restart = RestartOption::ALWAYS;
    let mut program = Program::build_from((&config_name, &config));

    // when
    let mut process = Process::start(&mut program.command);
    process.child.as_mut().unwrap().wait().unwrap();
    process.update_status(&config);

    // then
    assert!(process.should_restart);
}

static mut TICK: u64 = 0;

#[derive(Debug)]
pub struct Instant {
    seconds: u64,
}

impl Instant {
    pub fn now() -> Instant {
        Instant {
            seconds: unsafe { TICK },
        }
    }

    pub fn elapsed(&self) -> Duration {
        Duration {
            seconds: unsafe { TICK - self.seconds },
        }
    }
}

impl Instant {
    pub fn advance(seconds: u64) {
        unsafe {
            TICK += seconds;
        }
    }

    pub fn advance_from_duration(duration: Duration) {
        unsafe {
            TICK += duration.seconds;
        }
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct Duration {
    seconds: u64,
}

impl Duration {
    pub(crate) fn from_secs(seconds: u64) -> Duration { Duration { seconds } }
}

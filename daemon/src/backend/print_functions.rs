use std::collections::HashMap;
use std::io::Error;
use std::process::Child;

use logger::{debug, info};

use super::Program;
use crate::config::ProgramConfig;

pub fn print_programs(msg: &str, programs: &HashMap<String, ProgramConfig>) {
    debug!("---- {msg}");
    let mut programs = programs.keys().collect::<Vec<_>>();

    programs.sort();
    programs.iter().for_each(|p| debug!("  {p}"));
}

pub fn print_process(program: &Program, process: &Result<Child, Error>, i: usize) {
    let pid_or_error = match process {
        Ok(child) => child.id().to_string(),
        Err(err) => err.to_string(),
    };
    info!(
        "{}[{}]: {:?} [{}]",
        program.config_name, i, program.status[i], pid_or_error
    );
}

pub fn print_processes(programs: &HashMap<String, Program>) {
    programs.iter().for_each(|(_, program)| {
        program
            .processes
            .iter()
            .enumerate()
            .for_each(|(i, process)| {
                print_process(program, process, i);
            })
    })
}

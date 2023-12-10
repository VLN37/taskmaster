use std::collections::HashMap;

use logger::debug;

use super::Program;
use crate::config::ProgramConfig;

pub fn print_programs(msg: &str, programs: &HashMap<String, ProgramConfig>) {
    debug!("---- {msg}");
    let mut programs = programs.keys().collect::<Vec<_>>();

    programs.sort();
    programs.iter().for_each(|p| debug!("  {p}"));
}

pub fn print_processes(programs: &HashMap<String, Program>) -> String {
    let mut dump = String::from("Process dump\n");
    for (_, program) in programs.iter() {
        for (i, process) in program.processes.iter().enumerate() {
            dump.push_str(&format!("{:15}[{}]: {}\n", program.config_name, i, process));
        }
    }
    dump.pop();
    dump
}

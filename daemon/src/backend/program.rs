use std::cmp::Ordering;
use std::env;
use std::fs::{File, OpenOptions};

use super::process::Process;
use crate::config::structs::{IOHandler, KnownHandler};
use crate::config::ProgramConfig;

pub struct Program {
    pub config_name: String,
    pub config:      ProgramConfig,
    pub stdout:      Vec<Option<File>>,
    pub stderr:      Vec<Option<File>>,
    pub processes:   Vec<Process>,
}

impl Program {
    pub fn build_from(
        (config_name, command_config): (&String, &ProgramConfig),
    ) -> Program {
        Program {
            config_name: config_name.to_string(),
            config:      command_config.clone(),
            processes:   vec![],
            stdout:      vec![],
            stderr:      vec![],
        }
    }

    pub fn create_output_files(&mut self) {
        for i in 0..self.config.processes {
            let mut opts = OpenOptions::new();
            opts.create(true).write(true).append(true).read(true);
            if let IOHandler::FILE(filename) = &self.config.stdout {
                self.stdout.push(Some(opts.open(filename).unwrap()));
            }
            if let IOHandler::KNOWN(handler) = &self.config.stdout {
                match handler {
                    KnownHandler::DEFAULT => {
                        let file = Some(Self::default_stdout(&self.config, i));
                        self.stdout.push(file);
                    }
                    KnownHandler::DISCARD => self.stdout.push(None),
                }
            }
        }

        for i in 0..self.config.processes {
            let mut opts = OpenOptions::new();
            opts.create(true).write(true).append(true).read(true);
            if let IOHandler::FILE(filename) = &self.config.stderr {
                self.stderr.push(Some(opts.open(filename).unwrap()));
            }
            if let IOHandler::KNOWN(handler) = &self.config.stderr {
                match handler {
                    KnownHandler::DEFAULT => {
                        let file = Some(Self::default_stderr(&self.config, i));
                        self.stderr.push(file);
                    }
                    KnownHandler::DISCARD => self.stderr.push(None),
                }
            }
        }
    }

    fn default_stdout(config: &ProgramConfig, process_id: usize) -> File {
        let name = format!(
            "{}/logs/{}/p{}/stdout",
            env::current_dir().unwrap().display(),
            config.command,
            process_id
        );
        File::options()
            .create(true)
            .append(true)
            .open(name)
            .unwrap()
    }

    fn default_stderr(config: &ProgramConfig, process_id: usize) -> File {
        let name = format!(
            "{}/logs/{}/p{}/stderr",
            env::current_dir().unwrap().display(),
            config.command,
            process_id
        );
        File::options()
            .create(true)
            .append(true)
            .open(name)
            .unwrap()
    }

    pub fn update_process_status(&mut self) {
        self.processes.iter_mut().for_each(|p| {
            p.update_status(&self.config);
            if p.should_restart {
                p.restart();
            } else if p.should_try_again {
                p.try_start_again();
            }
        })
    }

    pub fn update_process_count(&mut self) {
        let current_count = self.processes.len();
        let desired_count = self.config.processes;
        match current_count.cmp(&desired_count) {
            Ordering::Less => {
                for _ in 0..desired_count - current_count {
                    let mut process = self.create_process();
                    process.start();
                    self.processes.push(process);
                }
            }
            Ordering::Greater => {
                for _ in 0..current_count - desired_count {
                    self.processes.pop();
                }
            }
            _ => {}
        }
    }

    fn create_process(&self) -> Process { Process::new(&self.config) }
}

#[cfg(test)]
mod tests {
    use super::Program;
    use crate::config::ProgramConfig;

    #[test]
    fn test_program() {
        let mut config = ProgramConfig::new();
        config.command = String::from("echo");
        config.args = vec![String::from("test")];

        let mut program = Program::build_from((&String::from("echo"), &config));
        program.update_process_count();
        program.update_process_status();

        assert_eq!(program.processes.len(), 1);
    }

    #[test]
    fn test_update_process_count() {
        let mut config = ProgramConfig::new();
        config.command = String::from("echo");
        config.args = vec![String::from("test")];
        let mut program = Program::build_from((&String::from("echo"), &config));
        program.update_process_count();
        assert_eq!(program.processes.len(), 1);
        program.config.processes = 3;
        program.update_process_count();
        assert_eq!(program.processes.len(), 3);
        program.config.processes = 1;
        program.update_process_count();
        assert_eq!(program.processes.len(), 1);
    }
}

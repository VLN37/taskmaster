use std::error::Error;

use daemon::defs::DFL_CONFIG_FILE;
use daemon::taskmaster::{Status, TaskMaster};
use logger::info;

fn get_config_file() -> Result<String, Box<dyn Error>> {
    let mut arguments: Vec<String> = std::env::args().collect();
    let mut config_file = DFL_CONFIG_FILE.into();

    if arguments.len() > 2 {
        return Err("invalid number of arguments".into());
    }
    if arguments.len() > 1 {
        config_file = arguments.pop().unwrap();
    }
    Ok(config_file)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = get_config_file()?;
    let mut taskmaster = TaskMaster::new();

    '_config: loop {
        info!("Configuring...");
        match taskmaster.status {
            Status::Starting => taskmaster.build(&config_file)?,
            Status::Reloading => taskmaster.reload()?,
            Status::Active => info!("All Good!"),
        }
        // while taskmaster.serve_routine().is_ok() {}
        '_main: loop {
            taskmaster.backend.update_processes_status();
            match taskmaster.serve_routine() {
                Ok(_) => match taskmaster.status {
                    Status::Reloading => break '_main,
                    _ => continue,
                },
                Err(_) => break '_main,
            }
        }
    }
}

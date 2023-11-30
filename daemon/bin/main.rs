use std::error::Error;

use daemon::taskmaster::{Status, TaskMaster};
use logger::info;

fn main() -> Result<(), Box<dyn Error>> {
    let mut taskmaster = TaskMaster::new();

    '_config: loop {
        info!("Configuring...");
        match taskmaster.status {
            Status::Starting => taskmaster.build()?,
            Status::Reloading => taskmaster.reload()?,
            Status::Active => info!("All Good!"),
        }
        // while taskmaster.serve_routine().is_ok() {}
        '_main: loop {
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

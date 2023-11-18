mod taskmaster;

use std::error::Error;
use taskmaster::{Status, TaskMaster};

fn main() -> Result<(), Box<dyn Error>> {
    let mut taskmaster = TaskMaster::new();

    '_config: loop {
        println!("Configuring...");
        match taskmaster.status {
            Status::Starting => taskmaster.build()?,
            Status::Reloading => taskmaster.reload()?,
            Status::Active => println!("All Good!"),
        }
        // while taskmaster.serve_routine().is_ok() {}
        '_main: loop {
            match taskmaster.serve_routine() {
                Ok(_) => continue,
                Err(_) => break '_main,
            }
        }
    }
}

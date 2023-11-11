use masterlib::daemon::{config::Program, TaskMasterConfig};
use std::fs;

fn main() {
    let var = Program::new();
    println!("{:?}", var);
    let var = TaskMasterConfig::from(fs::File::open(masterlib::CONFIG_PATH).unwrap());
    println!("{:?}", var);
}

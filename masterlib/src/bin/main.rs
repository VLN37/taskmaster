use std::fs;

use masterlib::daemon::config::Program;
use masterlib::daemon::TaskMasterConfig;

fn main() {
    let var = Program::new();
    println!("{:?}", var);
    let var = TaskMasterConfig::from(fs::File::open(masterlib::CONFIG_PATH).unwrap());
    println!("{:?}", var);
}

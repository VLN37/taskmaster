use std::{fs, process};

use masterlib::daemon::TaskMasterConfig;

fn main() {
    let file = fs::File::open(masterlib::CONFIG_PATH).expect("Could not open file.");
    let config = match TaskMasterConfig::read(file) {
        Ok(r) => r,
        Err(err) => panic!("Fuck: {:?}", err),
    };
    let program = &config.programs["echo"];
    process::Command::new(&program.command)
        .arg(&program.args)
        .env("ANSWER", "42")
        .spawn()
        .expect("fuck");
    // std::io::stdout()
    //     .write_all(&output.unwrap().stdout)
    //     .unwrap();
}

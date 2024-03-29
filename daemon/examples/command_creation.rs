use std::{fs, process};

use daemon::defs::DFL_CONFIG_FILE;
use daemon::TaskMasterConfig;

fn main() {
    let file = fs::File::open(DFL_CONFIG_FILE).expect("Could not open file.");
    let config = match TaskMasterConfig::read(file) {
        Ok(r) => r,
        Err(err) => panic!("Fuck: {:?}", err),
    };
    let program = &config.programs["echo"];
    process::Command::new(&program.command)
        .args(program.args.to_vec())
        .env("ANSWER", "42")
        .spawn()
        .expect("fuck");
    // std::io::stdout()
    //     .write_all(&output.unwrap().stdout)
    //     .unwrap();
}

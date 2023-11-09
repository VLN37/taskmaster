use masterlib::config;
use std::io::Write;
use std::process;

fn main() {
    let file = std::fs::File::open("config.yml").expect("Could not open file.");
    let config = config::read(file);
    let program = &config.programs["echo"];
    let output = process::Command::new(&program.command)
        .arg(&program.args)
        .output();
    std::io::stdout()
        .write_all(&output.unwrap().stdout)
        .unwrap();
}

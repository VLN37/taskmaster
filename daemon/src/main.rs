use std::process;
use std::io::Write;
use masterlib::config;

fn main() {
    let file = std::fs::File::open("config.yml").expect("Could not open file.");
    let config = config::read(file);
    let program = &config.programs["echo"];
    let output = process::Command::new(&program.command).arg("This compiles!").output();
    std::io::stdout().write_all(&output.unwrap().stdout).unwrap();
}

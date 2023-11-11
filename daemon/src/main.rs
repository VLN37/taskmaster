use masterlib::daemon::config;
use std::io::{Write, Read};
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::{fs, process};

fn main() {
    let file = std::fs::File::open("config.yml").expect("Could not open file.");
    let config = match config::read(file) {
        Ok(r) => r,
        Err(err) => panic!("Fuck: {:?}", err),
    };
    let program = &config.programs["echo"];
    let output = process::Command::new(&program.command)
        .arg(&program.args)
        .output();
    std::io::stdout()
        .write_all(&output.unwrap().stdout)
        .unwrap();

    let socket = Path::new(masterlib::SOCKET_PATH);
    if socket.exists() {
        fs::remove_file(&socket).unwrap();
        println!("previous socket removed")
    }
    let listener = match UnixListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };
    println!("server started");

    for stream in listener.incoming() {
        let mut buffer = String::new();
        stream.unwrap().read_to_string(&mut buffer).unwrap();
        println!("Client: {}", buffer);
    }
}

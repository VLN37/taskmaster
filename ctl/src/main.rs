use std::{os::unix::net::UnixStream, io::Write};

use masterlib::config;

fn main() {
    println!("Hello, world!");
    let val = config::RestartOption::ALWAYS;
    println!("{:?}", val);

    let mut stream = UnixStream::connect(masterlib::SOCKET_PATH).unwrap();
    stream.write(b"asd").unwrap();
}

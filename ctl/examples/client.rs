use masterlib::daemon::config;
use std::{io::Write, os::unix::net::UnixStream};

/// cargo run -p ctl --example client
fn main() {
    println!("Hello, world!");
    let val = config::RestartOption::ALWAYS;
    println!("{:?}", val);

    let mut stream = UnixStream::connect(masterlib::SOCKET_PATH).unwrap();
    stream.write(b"STATUS bash").unwrap();
}

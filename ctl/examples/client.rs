use std::{io::Write, os::unix::net::UnixStream};

/// cargo run -p ctl --example client
fn main() {
    let mut stream = UnixStream::connect(masterlib::SOCKET_PATH).unwrap();
    stream.write_all(b"STATUS bash").unwrap();
}

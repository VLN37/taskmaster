use std::{io::Write, os::unix::net::UnixStream};
use masterlib;

fn main() {
    let mut stream = UnixStream::connect(masterlib::SOCKET_PATH).unwrap();
    stream.write(b"asd").unwrap();
}

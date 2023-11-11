use std::{io::Write, os::unix::net::UnixStream};

fn main() {
    let mut stream = UnixStream::connect(masterlib::SOCKET_PATH).unwrap();
    stream.write_all(b"asd").unwrap();
}

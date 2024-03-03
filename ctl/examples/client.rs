use std::io::Write;
use std::os::unix::net::UnixStream;

/// cargo run -p ctl --example client
fn main() {
    let mut stream = UnixStream::connect(common::DAEMON_SOCKET_PATH).unwrap();
    stream.write_all(b"STATUS bash").unwrap();
}

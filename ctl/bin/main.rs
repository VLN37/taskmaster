use std::error::Error;
use std::io::Write;
use std::os::unix::net::UnixStream;

use common::DAEMON_SOCKET_PATH;
use ctl::Client;
// use common::DFL_SERVER_SOCKET_PATH;
// use ctl::client::client_class::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let mut backend = UnixStream::connect(DAEMON_SOCKET_PATH).unwrap();
    backend.write_all(b"asd")?;
    println!("hello client");
    let mut client = Client::new();
    client.build()?;
    loop {
        client.serve_routine()?;
    }
    // Ok(())
}

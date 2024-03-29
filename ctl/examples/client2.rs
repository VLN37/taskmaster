use std::error::Error;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

use common::DAEMON_SOCKET_PATH;
use logger::{debug, error, info};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    buf.reserve(500);
    info!("Ready for input");
    loop {
        let mut stream = UnixStream::connect(DAEMON_SOCKET_PATH).unwrap();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => (),
            Err(e) => error!("stdin: {e}"),
        };
        let mut stuff = buf.clone();
        stuff.pop();
        debug!("{:10}: {}", "user", stuff);
        stream
            .write_all(buf.as_bytes())
            .unwrap_or_else(|e| error!("client error: {e:?}"));
        buf.clear();
        stream.shutdown(Shutdown::Write).unwrap();
        stream.read_to_string(&mut buf).unwrap();
        stream.shutdown(Shutdown::Read).unwrap();
        debug!("{:10}: {buf}", "server");
        buf.clear();
    }
}

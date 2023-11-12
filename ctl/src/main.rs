use std::{
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
};

fn main() {
    let mut buf = String::new();
    buf.reserve(500);
    println!("Ready for input");
    loop {
        let mut stream = UnixStream::connect(masterlib::SOCKET_PATH).unwrap();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => (),
            Err(e) => println!("stdin: {e}"),
        };
        buf.pop();
        stream
            .write_all(buf.as_bytes())
            .unwrap_or_else(|e| println!("client error: {e:?}"));
        buf.clear();
        stream.shutdown(Shutdown::Write).unwrap();
        stream.read_to_string(&mut buf).unwrap();
        stream.shutdown(Shutdown::Read).unwrap();
        println!("response: {buf}");
        buf.clear();
    }
}

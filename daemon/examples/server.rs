use std::io::Read;
use std::os::unix::net::UnixListener;
use std::path::Path;

// cargo run -p daemon --example server
fn main() {
    let socket = Path::new(masterlib::SOCKET_PATH);
    if socket.exists() {
        std::fs::remove_file(&socket).unwrap();
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

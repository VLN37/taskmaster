use std::io::Read;
use std::os::unix::net::UnixListener;
use std::path::Path;

// cargo run -p daemon --example server
fn main() {
    let socket = Path::new(masterlib::SOCKET_PATH);
    if socket.exists() {
        std::fs::remove_file(socket).unwrap();
        println!("previous socket removed");
    }
    let listener = match UnixListener::bind(socket) {
        Ok(stream) => stream,
        Err(_) => panic!("failed to bind socket"),
    };
    println!("server started");

    for client in listener.incoming() {
        let mut buffer = String::new();
        client.unwrap().read_to_string(&mut buffer).unwrap();
        println!("Client: {}", buffer);
    }
}

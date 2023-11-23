use std::error::Error;
use std::io::Read;
use std::os::unix::net::UnixListener;
use std::path::Path;

use common::server::Server;
use daemon::BackEnd;

// cargo run -p daemon --example server
fn main() {
    let socket = Path::new(common::SOCKET_PATH);
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

// naive server
fn _other() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    server.build()?;
    let _backend = BackEnd::new();
    let epollfd = server.create_epoll()?;

    println!("epollfd {epollfd}");
    println!("Awaiting front-end connection");
    // server.accept();
    for conn in server.socket.incoming() {
        println!("CONNECTED");
        let mut _client = match conn {
            Ok(c) => c,
            Err(e) => {
                println!("connect failed {e:?}");
                continue;
            }
        };
    }
    Ok(())
}

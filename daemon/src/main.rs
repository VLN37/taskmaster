use std::error::Error;

use masterlib::daemon::server::Server;
use masterlib::daemon::BackEnd;

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    server.build()?;
    let backend = BackEnd::new();
    let epollfd = server.create_epoll()?;

    println!("epollfd {epollfd}");
    println!("Awaiting front-end connection");
    // server.accept();
    for conn in server.socket.incoming() {
        println!("CONNECTED");
        let mut client = match conn {
            Ok(c) => c,
            Err(e) => {
                println!("connect failed {e:?}");
                continue;
            }
        };
        backend.process(&mut client).unwrap_or_else(|x| println!("err: {x:?}"));
    }
    Ok(())
}

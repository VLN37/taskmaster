use core::time;
use std::error::Error;
use std::thread::sleep;

use masterlib::daemon::server::{Key, Server, SERVER_KEY};
use masterlib::daemon::BackEnd;

// poll server
fn main() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    let mut backend = BackEnd::new();

    server.build()?;
    println!("Server ready");

    loop {
        sleep(time::Duration::from_secs(1));
        println!("#{} AWAITING", server.key);
        server.epoll_wait()?;
        for ev in server.get_events() {
            let key: Key = ev.u64;
            match key {
                SERVER_KEY => {
                    if let Ok(client_key) = server.accept() {
                        println!("#{} ACCEPTED", server.key);
                        backend.clients.insert(server.key, client_key);
                    }
                }
                key => {
                    if (ev.events & libc::EPOLLIN as u32) != 0 {
                        server.recv(key)?;
                        println!("#{key} REQUEST READ");
                        server.modify_interest(key, Server::write_event(key))?;
                    } else if (ev.events & libc::EPOLLOUT as u32) != 0 {
                        server.send(key)?;
                        println!("#{key} RESPONSE SENT");
                    } else {
                        let ev = ev.events;
                        println!("Unexpected event: {}", ev);
                    }
                }
            };
        }
    }
    // Ok(())
}

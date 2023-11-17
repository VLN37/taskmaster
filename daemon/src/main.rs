use core::time;
use std::error::Error;
use std::os::fd::AsRawFd;
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
        for ev in &server.events {
            let key: Key = ev.u64;
            match key {
                SERVER_KEY => {
                    server.key += 1;
                    let client = match server.accept() {
                        Ok(c) => c,
                        Err(e) => {
                            println!("{e:?}");
                            continue;
                        }
                    };
                    println!("#{} ACCEPTED", server.key);
                    backend.clients.insert(server.key, client);
                }
                key => {
                    if (ev.events & libc::EPOLLIN as u32) != 0 {
                        backend.recv(key)?;
                        println!("#{key} REQUEST READ");
                        server.modify_interest(
                            backend.clients.get(&key).unwrap().as_raw_fd(),
                            Server::write_event(key),
                        )?;
                    }
                    if (ev.events & libc::EPOLLOUT as u32) != 0 {
                        backend.send(key)?;
                        println!("#{key} RESPONSE SENT");
                    }
                }
            };
        }
    }
    // Ok(())
}

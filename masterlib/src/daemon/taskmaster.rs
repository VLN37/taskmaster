pub mod status;

use std::thread::sleep;
use std::{io, time};

pub use self::status::Status;
use crate::daemon::request_factory::RequestFactory;
use crate::daemon::server::{Key, Server, SERVER_KEY};
use crate::daemon::BackEnd;

#[derive(Default)]
pub struct TaskMaster {
    pub server:  Server,
    pub backend: BackEnd,
    pub status:  Status,
    pub factory: RequestFactory,
}

impl TaskMaster {
    pub fn new() -> TaskMaster { Self::default() }

    pub fn build(&mut self) -> io::Result<()> {
        self.server.build()?;
        self.status = Status::Active;
        Ok(())
    }

    pub fn serve_routine(&mut self) -> io::Result<()> {
        sleep(time::Duration::from_secs(1));
        println!("#{} AWAITING", self.server.key);
        self.server.epoll_wait()?;
        for ev in self.server.get_events() {
            let key: Key = ev.u64;
            if key == SERVER_KEY {
                if self.server.accept().is_ok() {
                    println!("#{} ACCEPTED", self.server.key);
                }
                continue;
            }
            if (ev.events & libc::EPOLLIN as u32) != 0 {
                if let Ok(mut msg) = self.server.recv(key) {
                    self.factory.insert(key, &mut msg);
                    println!("#{key} REQUEST READ");
                    if let Some(request) = self.factory.parse(key) {
                        self.backend.clients.insert(key, request);
                        self.server.modify_interest(key, Server::write_event(key))?;
                    } else {
                        self.server.modify_interest(key, Server::read_event(key))?;
                    }
                } else {
                    self.server.clients.remove(&key);
                    continue;
                }
            } else if (ev.events & libc::EPOLLOUT as u32) != 0 {
                let msg = self.backend.get_response_for(key);
                self.server.send(key, &msg)?;
                // should close the request when Response object is finished
                // we will know if the backend is done with it, now we don't
                println!("#{key} RESPONSE SENT");
            } else {
                let ev = ev.events;
                println!("Unexpected event: {}", ev);
            }
        }
        Ok(())
    }

    pub fn reload(&mut self) -> io::Result<()> { todo!() }
}

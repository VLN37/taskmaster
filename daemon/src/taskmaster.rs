mod status;

pub use self::status::Status;
use masterlib::daemon::{
    server::{Key, Server, SERVER_KEY},
    BackEnd,
};
use std::{io, thread::sleep, time};

#[derive(Default)]
pub struct TaskMaster {
    pub server: Server,
    pub backend: BackEnd,
    pub status: Status,
}

impl TaskMaster {
    pub fn new() -> TaskMaster {
        Self::default()
    }

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
            match key {
                SERVER_KEY => {
                    if let Ok(client_key) = self.server.accept() {
                        println!("#{} ACCEPTED", self.server.key);
                        self.backend.clients.insert(self.server.key, client_key);
                    }
                }
                key => {
                    if (ev.events & libc::EPOLLIN as u32) != 0 {
                        self.server.recv(key)?;
                        println!("#{key} REQUEST READ");
                        self.server.modify_interest(key, Server::write_event(key))?;
                    } else if (ev.events & libc::EPOLLOUT as u32) != 0 {
                        self.server.send(key)?;
                        println!("#{key} RESPONSE SENT");
                    } else {
                        let ev = ev.events;
                        println!("Unexpected event: {}", ev);
                    }
                }
            };
        }
        Ok(())
    }

    pub fn reload(&mut self) -> io::Result<()> {
        todo!()
    }
}

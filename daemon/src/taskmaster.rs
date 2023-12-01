pub mod status;
use std::fs::File;
use std::io;

use common::server::{Key, RequestFactory, Server, SERVER_KEY};
use common::CONFIG_PATH;
use logger::{debug, error, info};
pub use status::Status;

use crate::signal_handling::install_sighup_handler;
use crate::BackEnd;

#[derive(Default)]
pub struct TaskMaster {
    pub server:  Server,
    pub backend: BackEnd,
    pub status:  Status,
    pub factory: RequestFactory,
}

impl TaskMaster {
    pub fn new() -> TaskMaster { TaskMaster::default() }

    pub fn build(&mut self) -> io::Result<()> {
        self.server.build()?;
        self.backend = BackEnd::new(File::open(CONFIG_PATH)?.into());

        self.backend.start();

        let ptr: *mut Status = &mut self.status;
        install_sighup_handler(move || unsafe {
            *ptr = Status::Reloading;
        });
        self.status = Status::Active;
        Ok(())
    }

    pub fn serve_routine(&mut self) -> io::Result<()> {
        info!("#{} AWAITING", self.server.key);
        self.server.epoll_wait()?;
        for ev in self.server.get_events() {
            let key: Key = ev.u64;
            if key == SERVER_KEY {
                if self.server.accept().is_ok() {
                    info!("#{} ACCEPTED", self.server.key);
                }
                continue;
            }
            if (ev.events & libc::EPOLLIN as u32) != 0 {
                if let Ok(mut msg) = self.server.recv(key) {
                    self.factory.insert(key, &mut msg);
                    info!("#{key} REQUEST READ");
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
                info!("#{key} RESPONSE SENT");
            } else {
                let ev = ev.events;
                error!("Unexpected event: {}", ev);
            }
        }
        Ok(())
    }

    pub fn reload(&mut self) -> io::Result<()> {
        if let Status::Reloading = self.status {
            debug!("Reloading!!!");
            self.backend
                .update(File::open(CONFIG_PATH)?.into())
                .expect("Failed to reload config");
            self.status = Status::Active;
        };
        Ok(())
    }
}

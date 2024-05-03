use std::fs::File;
use std::io;

use common::server::{Key, Server, ServerError, SERVER_KEY};
use common::DAEMON_SOCKET_PATH;
use logger::{debug, error, info};

use super::{RequestFactory, Status};
use crate::signal_handling::{install_sigchld_handler, install_sighup_handler};
use crate::BackEnd;

pub struct TaskMaster {
    pub server:      Server,
    pub backend:     BackEnd,
    pub status:      Status,
    pub factory:     RequestFactory,
    config_filename: String,
}

impl TaskMaster {
    #[allow(clippy::new_without_default)]
    pub fn new() -> TaskMaster {
        TaskMaster {
            server:          Server::new(DAEMON_SOCKET_PATH),
            backend:         BackEnd::default(),
            status:          Status::default(),
            factory:         RequestFactory::default(),
            config_filename: String::default(),
        }
    }

    pub fn build(&mut self, config_filename: &str) -> Result<(), ServerError> {
        self.server.build()?;
        let file = File::open(config_filename)?;
        self.config_filename = config_filename.into();
        self.backend = BackEnd::new(file.into());
        self.backend.start();

        let ptr: *mut Status = &mut self.status;
        install_sighup_handler(move || unsafe {
            *ptr = Status::Reloading;
        });

        let backend_ptr: *mut BackEnd = &mut self.backend;
        install_sigchld_handler(move || unsafe {
            (*backend_ptr).handle_dead_processes();
        });
        self.status = Status::Active;
        Ok(())
    }

    pub fn reload(&mut self) -> io::Result<()> {
        let file = File::open(&self.config_filename)?;
        if let Status::Reloading = self.status {
            debug!("Reloading!!!");
            self.backend
                .update(file.into())
                .expect("Failed to reload config");
            self.status = Status::Active;
        };
        Ok(())
    }

    pub fn serve_routine(&mut self) -> Result<(), ServerError> {
        // info!("#{} AWAITING", self.server.key);
        self.server.epoll_wait()?;
        // self.backend.dump_processes_status();
        for ev in self.server.get_events() {
            let key: Key = ev.u64;
            if key == SERVER_KEY {
                if self.server.accept().is_ok() {
                    info!("#{} CONNECTED", self.server.key);
                }
                continue;
            }
            if (ev.events & libc::EPOLLIN as u32) != 0 {
                if self.receive(key).is_err() {
                    self.server.clients.remove(&key);
                    continue;
                }
            } else if (ev.events & libc::EPOLLOUT as u32) != 0 {
                self.respond(key)?;
            } else {
                let ev = ev.events;
                error!("Unexpected event: {}", ev);
            }
        }
        Ok(())
    }

    fn receive(&mut self, key: Key) -> Result<(), ServerError> {
        let mut msg = self.server.recv(key)?;
        self.factory.insert(key, &mut msg);
        if let Some(request) = self.factory.parse(key) {
            self.backend.insert(key, request);
            self.server.modify_interest(Server::write_event(key))?;
        } else {
            self.request_read(key)?;
        }
        Ok(())
    }

    fn request_read(&mut self, key: Key) -> Result<(), ServerError> {
        let event = match self.backend.is_attached(key) {
            true => Server::read_write_event(key),
            false => Server::read_event(key),
        };
        self.server.modify_interest(event)
    }

    fn respond(&mut self, key: Key) -> Result<(), ServerError> {
        // the connection is kept alive until dropped by frontend

        if let Some(response) = self.backend.get_response_for(key) {
            self.server.send(key, &response.message)?;
            self.request_read(key)?;
            info!("#{key} SENT");
        }
        Ok(())
    }
}
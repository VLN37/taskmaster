use std::collections::VecDeque;
use std::io::Write;
use std::os::unix::net::UnixStream;

use common::request::{ClientState, Request};
use common::server::{Key, Server, ServerError, SERVER_KEY};
use common::{CTL_SOCKET_PATH, DAEMON_SOCKET_PATH};
use libc::STDIN_FILENO;
use logger::{debug, info};

const BACK: Key = 1;

pub struct Client {
    pub server:  Server,
    pub backend: UnixStream,
    pub queries: VecDeque<String>,
    pub state:   ClientState,
}

impl Client {
    pub fn new() -> Client {
        Client {
            server:  Server::new(CTL_SOCKET_PATH),
            backend: UnixStream::connect(DAEMON_SOCKET_PATH).unwrap(),
            queries: VecDeque::new(),
            state:   ClientState::default(),
        }
    }

    pub fn build(&mut self) -> Result<(), ServerError> {
        self.server.build()?;

        let stdin = STDIN_FILENO as Key;
        self.server.add_interest(Server::read_event(stdin))?;

        self.backend.set_nonblocking(true)?;
        self.server.clients.insert(1, self.backend.try_clone()?);
        self.server.add_interest(Server::write_event(BACK))?;
        Ok(())
    }

    pub fn serve_routine(&mut self) -> Result<(), ServerError> {
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
                self.receive(key)?;
            }
            if (ev.events & libc::EPOLLOUT as u32) != 0 && !self.queries.is_empty() {
                self.query()?;
            }
        }
        Ok(())
    }

    fn query(&mut self) -> Result<(), ServerError> {
        if let Some(query) = self.queries.pop_front() {
            self.backend.write_all(query.as_bytes())?;
            self.server.modify_interest(Server::read_event(BACK))?;
        }
        Ok(())
    }

    fn receive(&mut self, key: Key) -> Result<(), ServerError> {
        let msg = self.server.recv(key)?;
        if msg.trim().is_empty() {
            return Ok(());
        }
        if key == STDIN_FILENO as Key {
            let mut request = Request::from(&msg);
            request.client_key = key;
            request.state = self.state.clone();
            let res = request.validate();
            if res.is_err() {
                println!("Error: {}", res.unwrap_err());
            } else {
                self.queries.push_back(msg);
                self.request_write(BACK)?;
                debug!("current queries: {:?}", &self.queries);
            }
        } else {
            println!("backend: {msg}");
        }
        Ok(())
    }

    fn request_write(&mut self, key: Key) -> Result<(), ServerError> {
        let ev = match &self.state {
            ClientState::Attached(_) => Server::read_write_event(key),
            ClientState::Unattached => Server::write_event(key),
        };
        self.server.modify_interest(ev)
    }
}

impl Default for Client {
    fn default() -> Client {
        Client {
            server:  Server::new(CTL_SOCKET_PATH),
            backend: UnixStream::connect(DAEMON_SOCKET_PATH).unwrap(),
            queries: VecDeque::new(),
            state:   ClientState::default(),
        }
    }
}

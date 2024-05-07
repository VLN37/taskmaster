use std::collections::VecDeque;
use std::io::Write;
use std::os::unix::net::UnixStream;

use common::server::{Key, Server, ServerError, SERVER_KEY, STDIN_KEY};
use common::{ClientState, Cmd, Request, CTL_SOCKET_PATH, DAEMON_SOCKET_PATH};
use logger::{debug, info};

const BACKEND_KEY: Key = 1;

pub struct Client {
    pub server:  Server,
    pub backend: UnixStream,
    pub queries: VecDeque<String>,
    pub state:   ClientState,
}

impl Client {
    #[allow(clippy::new_without_default)]
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

        self.server.add_interest(Server::read_event(STDIN_KEY))?;

        self.backend.set_nonblocking(true)?;
        self.server.clients.insert(1, self.backend.try_clone()?);
        self.server.add_interest(Server::write_event(BACKEND_KEY))?;
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
            self.server
                .modify_interest(Server::read_event(BACKEND_KEY))?;
        }
        Ok(())
    }

    fn build_request(&self, k: Key, raw: &str) -> Request {
        let mut request = Request::from(raw);
        request.client_key = k;
        request.state = self.state.clone();
        if (request.state != ClientState::Unattached
            && request.command != Cmd::Unattach)
        {
            request.command = Cmd::Other(request.command.into());
        }
        request
    }

    fn receive(&mut self, key: Key) -> Result<(), ServerError> {
        let msg = self.server.recv(key)?;
        if msg.trim().is_empty() {
            return Ok(());
        }
        match key {
            STDIN_KEY => {
                let mut request = self.build_request(key, &msg);
                if request.is_valid() {
                    self.queries.push_back(msg);
                    self.request_write(BACKEND_KEY)?;
                    debug!("current queries: {:?}", &self.queries);
                } else {
                    println!("Error: {}", request.error.unwrap());
                }
            }
            _ => {
                println!("backend: {msg}");
                if msg.ends_with("Attach successful!") {
                    println!("frontend attached");
                    self.state = ClientState::Attached("backend knows".into());
                }
                if msg.ends_with("Unattach successful!") {
                    println!("frontend unattached");
                    self.state = ClientState::Unattached;
                }
            }
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

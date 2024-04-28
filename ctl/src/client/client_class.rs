use std::collections::VecDeque;
use std::io::Write;
use std::os::unix::net::UnixStream;

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

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ClientState {
    #[default]
    Unattached,
    Attached(String),
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

    pub fn query(&mut self) -> Result<(), ServerError> {
        let query = self.queries.pop_front().unwrap();
        // let mut request = Request::from(&query);
        // request.state = self.state.clone();
        self.backend.write_all(query.as_bytes())?;
        self.server.modify_interest(Server::read_event(BACK))?;
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
                let msg = self.server.recv(key)?;
                if key == STDIN_FILENO as Key {
                    if msg.trim().is_empty() {
                        continue;
                    }
                    self.queries.push_back(msg);
                    self.server.modify_interest(Server::write_event(BACK))?;
                    debug!("current queries: {:?}", &self.queries)
                } else {
                    println!("backend: {msg}");
                }
            }
            if (ev.events & libc::EPOLLOUT as u32) != 0 && !self.queries.is_empty() {
                self.query()?;
            }
        }
        Ok(())
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

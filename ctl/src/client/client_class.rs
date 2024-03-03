use std::collections::VecDeque;
use std::io::{self, Write};
use std::os::unix::net::UnixStream;

use common::server::{Key, Server, SERVER_KEY};
use common::{CTL_SOCKET_PATH, DAEMON_SOCKET_PATH};
use libc::STDIN_FILENO;
use logger::info;

pub struct Client {
    pub server:  Server,
    pub backend: UnixStream,
    pub queries: VecDeque<String>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            server:  Server::new(CTL_SOCKET_PATH),
            backend: UnixStream::connect(DAEMON_SOCKET_PATH).unwrap(),
            queries: VecDeque::new(),
        }
    }

    pub fn build(&mut self) -> io::Result<()> {
        self.server.build()?;

        let stdin = STDIN_FILENO as Key;
        self.server.add_interest(stdin, Server::read_event(stdin))?;

        self.backend.set_nonblocking(true)?;
        // self.queries.push_back("oeoeoeoe".into());
        // self.query()?;
        self.server.clients.insert(1, self.backend.try_clone()?);
        // self.server.add_interest(1, Server::read_event(1))?;
        self.server.add_interest(1, Server::write_event(1))?;
        Ok(())
    }

    pub fn query(&mut self) -> io::Result<()> {
        let query = self.queries.pop_front().unwrap();
        self.backend.write_all(query.as_bytes())?;
        Ok(())
    }

    pub fn serve_routine(&mut self) -> io::Result<()> {
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
                    println!("EPOLLIN {key}");
                    println!("query: {}", msg);
                    self.queries.push_back(msg);
                    self.server.modify_interest(1, Server::write_event(1))?;
                    println!("{:?}", &self.queries);
                } else {
                    dbg!(msg);
                }
                // println!("{}", msg);
            }
            if (ev.events & libc::EPOLLOUT as u32) != 0 {
                println!("EPOLLOUT {key}");
                if !self.queries.is_empty() {
                    self.query()?;
                }
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
        }
    }
}

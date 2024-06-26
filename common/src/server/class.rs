use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

use libc::{
    epoll_event,
    EPOLLIN,
    EPOLLONESHOT,
    EPOLLOUT,
    EPOLL_CTL_ADD,
    EPOLL_CTL_DEL,
    EPOLL_CTL_MOD,
};
use logger::{debug, error, info, warning};

use super::{Key, SERVER_KEY, STDIN_KEY};
use crate::syscall;

pub struct Server {
    pub socket:  UnixListener,
    pub events:  Vec<epoll_event>,
    pub pollfd:  RawFd,
    pub clients: HashMap<u64, UnixStream>,
    pub key:     u64,
    ready:       bool,
}

impl Server {
    pub fn new(socket_path: &str) -> Server {
        if Path::new(socket_path).exists() {
            std::fs::remove_file(socket_path).unwrap();
            info!("previous socket removed");
        }
        let socket = match UnixListener::bind(socket_path) {
            Ok(val) => val,
            Err(e) => panic!("{e:?}"),
        };
        let mut resourcelimit = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        syscall!(getrlimit(libc::RLIMIT_NOFILE, &mut resourcelimit)).unwrap();

        Server {
            socket,
            events: Vec::with_capacity(resourcelimit.rlim_cur as usize),
            pollfd: RawFd::default(),
            clients: HashMap::new(),
            key: SERVER_KEY,
            ready: false,
        }
    }

    pub fn build(&mut self) -> super::Result<()> {
        self.pollfd = match self.create_epoll() {
            Ok(fd) => fd,
            Err(err) => panic!("panic {err}"),
        };
        let sfd = self.socket.as_raw_fd();
        syscall!(epoll_ctl(self.pollfd, EPOLL_CTL_ADD, sfd, &mut Self::listen()))?;
        self.ready = true;
        Ok(())
    }

    pub fn accept(&mut self) -> super::Result<Key> {
        match self.socket.accept() {
            Ok((stream, _addr)) => {
                self.key += 1;
                stream.set_nonblocking(true)?;
                self.clients.insert(self.key, stream);
                self.add_interest(Self::read_event(self.key))?;
                Ok(self.key)
            }
            Err(e) => {
                error!("{e:?}");
                Err(e.into())
            }
        }
    }

    pub fn get_events(&self) -> Vec<epoll_event> { self.events.clone() }

    pub fn create_epoll(&mut self) -> super::Result<RawFd> {
        let fd = syscall!(epoll_create1(0))?;
        if let Ok(flags) = syscall!(fcntl(fd, libc::F_GETFD)) {
            syscall!(fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC))?;
        }
        Ok(fd)
    }

    pub fn epoll_wait(&mut self) -> super::Result<()> {
        self.events.clear();
        let res = match syscall!(epoll_wait(
            self.pollfd,
            self.events.as_mut_ptr(),
            1024,
            1000 as libc::c_int,
        )) {
            Ok(res) => res,
            Err(e) => match e.kind() {
                io::ErrorKind::Interrupted => return Ok(()),
                _ => panic!("error during epoll wait: {}", e),
            },
        };

        // safe  as long as the kernel does nothing wrong - copied from mio
        unsafe { self.events.set_len(res as usize) };
        Ok(())
    }

    pub fn add_interest(&self, mut event: epoll_event) -> super::Result<()> {
        let mut fd: i32 = 0;
        let key = event.u64 as Key;
        if key != STDIN_KEY {
            fd = self.clients.get(&key).unwrap().as_raw_fd();
        }
        syscall!(epoll_ctl(self.pollfd, EPOLL_CTL_ADD, fd, &mut event))?;
        Ok(())
    }

    pub fn modify_interest(&self, mut event: epoll_event) -> super::Result<()> {
        let mut fd: i32 = 0;
        let key = event.u64 as Key;
        if key != STDIN_KEY {
            fd = self.clients.get(&key).unwrap().as_raw_fd();
        }
        syscall!(epoll_ctl(self.pollfd, EPOLL_CTL_MOD, fd, &mut event))?;
        Ok(())
    }

    pub fn remove_interest(&self, key: Key) -> super::Result<()> {
        let mut fd: i32 = 0;
        if key != STDIN_KEY {
            fd = self.clients.get(&key).unwrap().as_raw_fd();
        }
        syscall!(epoll_ctl(self.pollfd, EPOLL_CTL_DEL, fd, std::ptr::null_mut()))?;
        Ok(())
    }

    fn listen() -> epoll_event {
        epoll_event {
            events: EPOLLIN as u32,
            u64:    SERVER_KEY,
        }
    }

    pub fn read_write_event(key: Key) -> epoll_event {
        epoll_event {
            events: (EPOLLOUT | EPOLLIN) as u32,
            u64:    key,
        }
    }

    pub fn read_event(key: Key) -> epoll_event {
        epoll_event {
            events: (EPOLLONESHOT | EPOLLIN) as u32,
            u64:    key,
        }
    }

    pub fn write_event(key: Key) -> epoll_event {
        epoll_event {
            events: (EPOLLONESHOT | EPOLLOUT) as u32,
            u64:    key,
        }
    }

    pub fn fixed_write(key: Key) -> epoll_event {
        epoll_event {
            events: EPOLLOUT as u32,
            u64:    key,
        }
    }

    pub fn fixed_read(key: Key) -> epoll_event {
        epoll_event {
            events: EPOLLIN as u32,
            u64:    key,
        }
    }

    fn recv_stdin(&self) -> super::Result<String> {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        let stdin = STDIN_KEY;
        debug!("{:13}: |{}|", "STDIN", buf.escape_default());
        self.modify_interest(Server::read_event(stdin))?;
        Ok(buf)
    }

    pub fn recv(&mut self, key: Key) -> super::Result<String> {
        if key == STDIN_KEY {
            return self.recv_stdin();
        }

        let mut buf = [0_u8; 1024];
        if let Some(client) = self.clients.get_mut(&key) {
            match client.read(&mut buf) {
                Ok(bytes) => {
                    if bytes == 0 {
                        warning!("#{key} DROPPED BY CLIENT (READ 0 BYTES)");
                        return Err(io::Error::from_raw_os_error(32).into());
                    }
                    let res = String::from_utf8(buf[0..bytes].into()).unwrap();
                    debug!("#{key} RECEIVED: |{}|", res.escape_default());
                    Ok(res)
                }
                Err(e) => {
                    self.remove_interest(key)?;
                    self.clients.remove(&key);
                    warning!("{key} removed from server due to: {e}");
                    Err(e.into())
                }
            }
        } else {
            panic!("server: invalid key {key}");
        }
    }

    pub fn send(&mut self, key: Key, msg: &String) -> super::Result<()> {
        if let Some(client) = self.clients.get_mut(&key) {
            client.write_all(msg.as_bytes())?;
        } else {
            error!("server: invalid key {key}");
        }
        Ok(())
    }
}

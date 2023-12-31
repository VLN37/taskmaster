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

use super::{Key, SERVER_KEY};
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
    pub fn new() -> Server { Server::default() }

    pub fn build(&mut self) -> io::Result<()> {
        self.pollfd = match self.create_epoll() {
            Ok(fd) => fd,
            Err(err) => panic!("panic {err}"),
        };
        let sfd = self.socket.as_raw_fd();
        syscall!(epoll_ctl(self.pollfd, EPOLL_CTL_ADD, sfd, &mut Self::listen()))?;
        self.ready = true;
        Ok(())
    }

    pub fn accept(&mut self) -> io::Result<Key> {
        match self.socket.accept() {
            Ok((stream, _addr)) => {
                self.key += 1;
                stream.set_nonblocking(true)?;
                self.clients.insert(self.key, stream);
                self.add_interest(self.key, Self::read_event(self.key))?;
                Ok(self.key)
            }
            Err(e) => {
                error!("{e:?}");
                Err(e)
            }
        }
    }

    pub fn get_events(&self) -> Vec<epoll_event> { self.events.clone() }

    pub fn create_epoll(&mut self) -> io::Result<RawFd> {
        let fd = syscall!(epoll_create1(0))?;
        if let Ok(flags) = syscall!(fcntl(fd, libc::F_GETFD)) {
            syscall!(fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC))?;
        }
        Ok(fd)
    }

    pub fn epoll_wait(&mut self) -> io::Result<()> {
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

    pub fn add_interest(&self, key: Key, mut event: epoll_event) -> io::Result<()> {
        let fd = self.clients.get(&key).unwrap().as_raw_fd();
        syscall!(epoll_ctl(self.pollfd, EPOLL_CTL_ADD, fd, &mut event))?;
        Ok(())
    }

    pub fn modify_interest(&self, key: Key, mut event: epoll_event) -> io::Result<()> {
        let fd = self.clients.get(&key).unwrap().as_raw_fd();
        syscall!(epoll_ctl(self.pollfd, EPOLL_CTL_MOD, fd, &mut event))?;
        Ok(())
    }

    pub fn remove_interest(&self, key: Key) -> io::Result<()> {
        let fd = self.clients.get(&key).unwrap().as_raw_fd();
        syscall!(epoll_ctl(self.pollfd, EPOLL_CTL_DEL, fd, std::ptr::null_mut()))?;
        Ok(())
    }

    fn listen() -> epoll_event {
        epoll_event {
            events: EPOLLIN as u32,
            u64:    SERVER_KEY,
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

    pub fn recv(&mut self, key: Key) -> io::Result<String> {
        let mut buf = String::new();
        if let Some(client) = self.clients.get_mut(&key) {
            match client.read_to_string(&mut buf) {
                Ok(bytes) => {
                    if bytes == 0 {
                        warning!("#{key} DROPPED BY CLIENT (READ 0 BYTES)");
                        return Err(io::Error::from_raw_os_error(32));
                    }
                    debug!("#{key} RECEIVED: |{}|", buf.escape_default());
                    Ok(buf)
                }
                Err(e) => {
                    self.remove_interest(key)?;
                    self.clients.remove(&key);
                    warning!("{key} removed from server due to: {e}");
                    Err(e)
                }
            }
        } else {
            panic!("server: invalid key {key}");
        }
    }

    pub fn send(&mut self, key: Key, msg: &String) -> io::Result<()> {
        if let Some(client) = self.clients.get_mut(&key) {
            client.write_all(msg.as_bytes())?;
            client.shutdown(std::net::Shutdown::Both)?;
        } else {
            error!("server: invalid key {key}");
        }
        Ok(())
    }
}

impl Default for Server {
    fn default() -> Server {
        if Path::new(crate::SOCKET_PATH).exists() {
            std::fs::remove_file(crate::SOCKET_PATH).unwrap();
            info!("previous socket removed");
        }
        let socket = match UnixListener::bind(crate::SOCKET_PATH) {
            Ok(val) => val,
            Err(e) => panic!("{e:?}"),
        };
        Server {
            socket,
            events: Vec::with_capacity(1024),
            pollfd: 0,
            ready: false,
            key: SERVER_KEY,
            clients: HashMap::new(),
        }
    }
}

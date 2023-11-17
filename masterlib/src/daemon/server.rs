use crate::syscall;
use libc::{
    epoll_event, EPOLLIN, EPOLLONESHOT, EPOLLOUT, EPOLL_CTL_ADD, EPOLL_CTL_DEL,
    EPOLL_CTL_MOD,
};
use std::collections::HashMap;
use std::os::{
    fd::{AsRawFd, RawFd},
    unix::net::{UnixListener, UnixStream},
};
use std::{io, path::Path};

pub type Key = u64;
pub const SERVER_KEY: Key = 42;

pub struct Server {
    pub socket: UnixListener,
    pub events: Vec<epoll_event>,
    pub pollfd: RawFd,
    pub clients: HashMap<u64, UnixStream>,
    pub key: u64,
    ready: bool,
}
impl Server {
    pub fn new() -> Server {
        Server::default()
    }

    pub fn build(&mut self) -> io::Result<()> {
        self.pollfd = match self.create_epoll() {
            Ok(fd) => fd,
            Err(err) => panic!("panic {err}"),
        };
        self.add_interest(self.socket.as_raw_fd(), Self::read_event(SERVER_KEY))?;
        self.ready = true;
        Ok(())
    }

    pub fn accept(&self) -> io::Result<UnixStream> {
        match self.socket.accept() {
            Ok(client) => Ok(client.0),
            Err(e) => Err(e),
        }
    }

    pub fn accept2(&self) -> io::Result<UnixStream> {
        match self.socket.accept() {
            Ok((stream, _addr)) => {
                stream.set_nonblocking(true)?;
                self.add_interest(stream.as_raw_fd(), Self::read_event(self.key))?;
                Ok(stream)
            }
            Err(e) => {
                println!("{e:?}");
                Err(e)
            }
        }
    }

    pub fn create_epoll(&self) -> io::Result<RawFd> {
        let fd = syscall!(epoll_create1(0))?;
        if let Ok(flags) = syscall!(fcntl(fd, libc::F_GETFD)) {
            syscall!(fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC))?;
        }
        Ok(fd)
    }

    pub fn epoll_wait(&mut self) -> io::Result<()> {
        self.events.clear();
        self.modify_interest(self.socket.as_raw_fd(), Self::read_event(SERVER_KEY))?;
        let res = match syscall!(epoll_wait(
            self.pollfd,
            self.events.as_mut_ptr(),
            1024,
            1000 as libc::c_int,
        )) {
            Ok(v) => v,
            Err(e) => panic!("error during epoll wait: {}", e),
        };

        // safe  as long as the kernel does nothing wrong - copied from mio
        unsafe { self.events.set_len(res as usize) };
        Ok(())
    }

    pub fn add_interest(&self, fd: RawFd, mut event: epoll_event) -> io::Result<()> {
        // let var = event.u64;
        // println!("event{}\npollfd {}\n fd {fd}", var, self.pollfd);
        syscall!(epoll_ctl(
            self.pollfd as libc::c_int,
            EPOLL_CTL_ADD,
            fd,
            &mut event as *mut epoll_event
        ))
        .expect("interest");
        Ok(())
    }

    pub fn modify_interest(&self, fd: RawFd, mut event: epoll_event) -> io::Result<()> {
        syscall!(epoll_ctl(self.pollfd, EPOLL_CTL_MOD, fd, &mut event))?;
        Ok(())
    }

    pub fn remove_interest(&self, fd: RawFd) -> io::Result<()> {
        syscall!(epoll_ctl(
            self.pollfd,
            EPOLL_CTL_DEL,
            fd,
            std::ptr::null_mut()
        ))?;
        Ok(())
    }

    pub fn read_event(key: Key) -> epoll_event {
        epoll_event {
            events: (EPOLLONESHOT | EPOLLIN) as u32,
            u64: key,
        }
    }

    pub fn write_event(key: Key) -> epoll_event {
        epoll_event {
            events: (EPOLLONESHOT | EPOLLOUT) as u32,
            u64: key,
        }
    }
}

impl Default for Server {
    fn default() -> Server {
        if Path::new(crate::SOCKET_PATH).exists() {
            std::fs::remove_file(crate::SOCKET_PATH).unwrap();
            println!("previous socket removed");
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

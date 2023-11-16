use crate::syscall;
use std::os::{
    fd::RawFd,
    unix::net::{UnixListener, UnixStream},
};
use std::{io, path::Path};

pub struct Server {
    pub socket: UnixListener,
    pollfd: RawFd,
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
        self.ready = true;
        Ok(())
    }

    pub fn accept(&self) -> io::Result<UnixStream> {
        match self.socket.accept() {
            Ok(client) => Ok(client.0),
            Err(e) => Err(e),
        }
    }

    pub fn create_epoll(&self) -> io::Result<RawFd> {
        let fd = syscall!(epoll_create1(0))?;
        if let Ok(flags) = syscall!(fcntl(fd, libc::F_GETFD)) {
            syscall!(fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC))?;
        }
        Ok(fd)
    }

    pub fn add_interest(
        &self,
        fd: RawFd,
        mut event: libc::epoll_event,
    ) -> io::Result<()> {
        syscall!(epoll_ctl(self.pollfd, libc::EPOLL_CTL_ADD, fd, &mut event))?;
        Ok(())
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
            pollfd: 0,
            ready: false,
        }
    }
}

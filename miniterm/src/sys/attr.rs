use std::os::fd::{AsRawFd, BorrowedFd};
use std::{io, mem};

use common::syscall;

use crate::Termios;

pub fn get_terminal_attr(fd: BorrowedFd) -> io::Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        syscall!(tcgetattr(fd.as_raw_fd(), &mut termios))?;
        Ok(termios)
    }
}

pub fn set_terminal_attr(fd: BorrowedFd, termios: &Termios) -> io::Result<()> {
    syscall!(tcsetattr(fd.as_raw_fd(), libc::TCSANOW, termios)).and(Ok(()))
}

pub fn raw_terminal_attr(termios: &mut Termios) { unsafe { libc::cfmakeraw(termios) } }

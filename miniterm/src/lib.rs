mod sys;
mod term;

use std::io::{self, Write};
use std::os::fd::AsFd;

pub use libc::termios as Termios;
use sys::attr::{get_terminal_attr, raw_terminal_attr, set_terminal_attr};
pub use term::RawTerminal;

pub trait IntoRawMode: Write + AsFd + Sized {
    fn into_raw_mode(self) -> io::Result<RawTerminal<Self>>;
}

impl<W: Write + AsFd> IntoRawMode for W {
    fn into_raw_mode(self) -> io::Result<RawTerminal<W>> {
        let mut ios = get_terminal_attr(self.as_fd())?;
        let prev_ios = ios;

        raw_terminal_attr(&mut ios);
        set_terminal_attr(self.as_fd(), &ios)?;
        Ok(RawTerminal {
            prev_ios,
            output: self,
        })
    }
}

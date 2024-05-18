use std::io::{self, Write};
use std::ops;
use std::os::fd::AsFd;

use crate::sys::attr::set_terminal_attr;
use crate::Termios;

/// A terminal restorer, which keeps the previous state of the terminal, and
/// restores it, when dropped.
///
/// Restoring will entirely bring back the old TTY state.
pub struct RawTerminal<W: Write + AsFd> {
    pub prev_ios: Termios,
    pub output:   W,
}

impl<W: Write + AsFd> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        let _ = set_terminal_attr(self.output.as_fd(), &self.prev_ios);
    }
}

impl<W: Write + AsFd> ops::Deref for RawTerminal<W> {
    type Target = W;

    fn deref(&self) -> &W { &self.output }
}

impl<W: Write + AsFd> ops::DerefMut for RawTerminal<W> {
    fn deref_mut(&mut self) -> &mut W { &mut self.output }
}

impl<W: Write + AsFd> Write for RawTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.output.write(buf) }

    fn flush(&mut self) -> io::Result<()> { self.output.flush() }
}

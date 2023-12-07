use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Signal {
    /// Hangup
    SIGHUP = 1,
    /// Interrupt
    SIGINT,
    /// Quit
    SIGQUIT,
    /// Illegal instruction (not reset when caught)
    SIGILL,
    /// Trace trap (not reset when caught)
    SIGTRAP,
    /// Abort
    SIGABRT,
    /// Bus error
    SIGBUS,
    /// Floating point exception
    SIGFPE,
    /// Kill (cannot be caught or ignored)
    SIGKILL,
    /// User defined signal 1
    SIGUSR1,
    /// Segmentation violation
    SIGSEGV,
    /// User defined signal 2
    SIGUSR2,
    /// Write on a pipe with no one to read it
    SIGPIPE,
    /// Alarm clock
    SIGALRM,
    /// Software termination signal from kill
    SIGTERM,
    /// Stack fault (obsolete)
    SIGSTKFLT,
    /// To parent on child stop or exit
    SIGCHLD,
    /// Continue a stopped process
    SIGCONT,
    /// Sendable stop signal not from tty
    SIGSTOP,
    /// Stop signal from tty
    SIGTSTP,
    /// To readers pgrp upon background tty read
    SIGTTIN,
    /// Like TTIN if (tp->t_local&LTOSTOP)
    SIGTTOU,
    /// Urgent condition on IO channel
    SIGURG,
    /// Exceeded CPU time limit
    SIGXCPU,
    /// Exceeded file size limit
    SIGXFSZ,
    /// Virtual time alarm
    SIGVTALRM,
}

impl From<i32> for Signal {
    fn from(signal: i32) -> Self {
        match signal {
            1 => Signal::SIGHUP,
            2 => Signal::SIGINT,
            3 => Signal::SIGQUIT,
            4 => Signal::SIGILL,
            5 => Signal::SIGTRAP,
            6 => Signal::SIGABRT,
            7 => Signal::SIGBUS,
            8 => Signal::SIGFPE,
            9 => Signal::SIGKILL,
            10 => Signal::SIGUSR1,
            11 => Signal::SIGSEGV,
            12 => Signal::SIGUSR2,
            13 => Signal::SIGPIPE,
            14 => Signal::SIGALRM,
            15 => Signal::SIGTERM,
            16 => Signal::SIGSTKFLT,
            17 => Signal::SIGCHLD,
            18 => Signal::SIGCONT,
            19 => Signal::SIGSTOP,
            20 => Signal::SIGTSTP,
            21 => Signal::SIGTTIN,
            22 => Signal::SIGTTOU,
            23 => Signal::SIGURG,
            24 => Signal::SIGXCPU,
            25 => Signal::SIGXFSZ,
            26 => Signal::SIGVTALRM,
            _ => panic!("Unknown signal"),
        }
    }
}

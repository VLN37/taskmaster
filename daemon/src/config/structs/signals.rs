use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

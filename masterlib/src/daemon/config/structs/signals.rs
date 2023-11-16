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

use std::ffi::{c_int, c_long};

// N.B., for LLVM to recognize the void pointer type and by extension
//     functions like malloc(), we need to have it represented as i8* in
//     LLVM bitcode. The enum used here ensures this and prevents misuse
//     of the "raw" type by only having private variants. We need two
//     variants, because the compiler complains about the repr attribute
//     otherwise and we need at least one variant as otherwise the enum
//     would be uninhabited and at least dereferencing such pointers would
//     be UB.
#[cfg_attr(not(doc), repr(u8))] // work around https://github.com/rust-lang/rust/issues/90435
pub enum c_void {
    __variant1,
    __variant2,
}
pub type pid_t = i32;
pub type uid_t = u32;

pub const SA_ONSTACK: c_int = 0x08000000;
pub const SA_NOCLDWAIT: c_int = 0x00000002;
pub const SA_SIGINFO: c_int = 0x00000004;

// Internal, for casts to access union fields
#[repr(C)]
pub struct sifields_sigchld {
    si_pid: pid_t,
    si_uid: uid_t,
    si_status: c_int,
    si_utime: c_long,
    si_stime: c_long,
}
impl Copy for sifields_sigchld {}
impl Clone for sifields_sigchld {
    fn clone(&self) -> sifields_sigchld {
        *self
    }
}

// Internal, for casts to access union fields
#[repr(C)]
union sifields {
    _align_pointer: *mut c_void,
    sigchld: sifields_sigchld,
}
// Internal, for casts to access union fields. Note that some variants
// of sifields start with a pointer, which makes the alignment of
// sifields vary on 32-bit and 64-bit architectures.
#[repr(C)]
struct siginfo_f {
    _siginfo_base: [c_int; 3],
    sifields: sifields,
}

#[repr(C)]
#[derive(Debug)]
pub struct siginfo_t {
    pub si_signo: CInt,
    pub si_errno: CInt,
    pub si_code: CInt,
    #[doc(hidden)]
    #[deprecated(
        since = "0.2.54",
        note = "Please leave a comment on \
                  https://github.com/rust-lang/libc/pull/1316 if you're using \
                  this field"
    )]
    pub _pad: [CInt; 29],
    _align: [u64; 0],
}

pub type CInt = i32;
pub type SizeT = usize;
pub type SighandlerT = SizeT;

#[repr(C)]
pub struct sigset_t {
    #[cfg(target_pointer_width = "64")]
    __val: [u64; 16],
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: SighandlerT,
    pub sa_mask: sigset_t,
    pub sa_flags: CInt,
    pub sa_restorer: Option<extern "C" fn()>,
}

impl siginfo_t {
    unsafe fn sifields(&self) -> &sifields {
        &(*(self as *const siginfo_t as *const siginfo_f)).sifields
    }

    pub unsafe fn si_pid(&self) -> pid_t {
        self.sifields().sigchld.si_pid
    }

    pub unsafe fn si_uid(&self) -> uid_t {
        self.sifields().sigchld.si_uid
    }

    pub unsafe fn si_status(&self) -> c_int {
        self.sifields().sigchld.si_status
    }

    pub unsafe fn si_utime(&self) -> c_long {
        self.sifields().sigchld.si_utime
    }

    pub unsafe fn si_stime(&self) -> c_long {
        self.sifields().sigchld.si_stime
    }
}

extern "C" {
    pub fn sigemptyset(set: *mut sigset_t) -> CInt;

    pub fn sigaction(
        signum: CInt,
        act: *const sigaction,
        oldact: *mut sigaction,
    ) -> CInt;
}

use std::mem::MaybeUninit;
use std::ptr::null_mut;

use libc::{c_int, sigaction, sigemptyset, SA_SIGINFO};
use logger::debug;

use super::config::Signal;

static mut SIGHUP_CLOSURE: Option<Box<dyn FnMut(c_int, *mut libc::siginfo_t)>> = None;
static mut SIGCHLD_CLOSURE: Option<Box<dyn FnMut(c_int, *mut libc::siginfo_t)>> = None;

extern "C" fn signal_handler(
    sig: c_int,
    siginfo: *mut libc::siginfo_t,
    _: *mut libc::c_void,
) {
    unsafe {
        let mut closure = match Signal::from(sig) {
            Signal::SIGHUP => SIGHUP_CLOSURE.as_mut(),
            Signal::SIGCHLD => SIGCHLD_CLOSURE.as_mut(),
            _ => panic!("unknown signal received"),
        };
        if let Some(ref mut handler) = closure {
            handler(sig, siginfo);
        }
    }
}

pub fn install_signal_handler(
    signal: Signal,
    handler: impl FnMut(i32, *mut libc::siginfo_t) + 'static,
) {
    let mut action: sigaction = unsafe { MaybeUninit::zeroed().assume_init() };
    unsafe {
        match signal {
            Signal::SIGHUP => SIGHUP_CLOSURE = Some(Box::new(handler)),
            Signal::SIGCHLD => SIGCHLD_CLOSURE = Some(Box::new(handler)),
            _ => panic!(
                "Invalid signal to handle. If you want to handle a new signal, add it \
                 to the match statement in signal_handler.rs"
            ),
        };
    }
    action.sa_sigaction = signal_handler as usize;
    action.sa_flags = SA_SIGINFO;
    unsafe { sigemptyset(&mut action.sa_mask) };
    unsafe { sigaction(signal as i32, &action, null_mut::<sigaction>()) };
}

pub fn install_sighup_handler(
    handler: impl FnMut(i32, *mut libc::siginfo_t) + 'static,
) {
    install_signal_handler(Signal::SIGHUP, handler);
}

pub fn install_sigchld_handler(
    handler: impl FnMut(i32, *mut libc::siginfo_t) + 'static,
) {
    install_signal_handler(Signal::SIGCHLD, handler);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::taskmaster::{Status, TaskMaster};

    fn initialize() {
        unsafe {
            SIGHUP_CLOSURE = None;
            SIGCHLD_CLOSURE = None;
        };
    }

    #[test]
    fn sighup_handler_test() {
        initialize();
        let mut taskmaster = TaskMaster::new();
        let ptr: *mut Status = &mut taskmaster.status;
        unsafe {
            install_sighup_handler(move |_sig, _info| {
                *ptr = Status::Reloading;
            });

            let mut sig = MaybeUninit::zeroed().assume_init();

            assert_eq!(taskmaster.status, Status::Starting);
            signal_handler(libc::SIGHUP, &mut sig, null_mut());
            assert_eq!(taskmaster.status, Status::Reloading);
            taskmaster.status = Status::Active;
            signal_handler(libc::SIGHUP, &mut sig, null_mut());
            assert_eq!(taskmaster.status, Status::Reloading);
            taskmaster.status = Status::Active;
            signal_handler(libc::SIGHUP, &mut sig, null_mut());
            assert_eq!(taskmaster.status, Status::Reloading);
            taskmaster.status = Status::Active;
        }
    }

    #[test]
    fn sigchld_handler_test() {
        initialize();
        let mut alive = true;
        let ptr: *mut bool = &mut alive;
        unsafe {
            install_sigchld_handler(move |_sig, _info| *ptr = !*ptr);

            let mut sig = MaybeUninit::zeroed().assume_init();
            assert!(alive);
            signal_handler(libc::SIGCHLD, &mut sig, null_mut());
            assert!(!alive);
            signal_handler(libc::SIGCHLD, &mut sig, null_mut());
            assert!(alive);
        }
    }

    #[test]
    fn unmangled_executions_test() {
        initialize();
        let mut chld_business = String::from("chld_business");
        let mut hup_business = String::from("hup_business");
        let chld_ptr: *mut String = &mut chld_business;
        let hup_ptr: *mut String = &mut hup_business;
        unsafe {
            install_signal_handler(Signal::SIGHUP, move |_sig, _info| {
                *hup_ptr = "hup received".to_string();
                assert_eq!(_sig, Signal::SIGHUP as i32)
            });
            install_signal_handler(Signal::SIGCHLD, move |_sig, _info| {
                *chld_ptr = "child received".to_string();
                assert_eq!(_sig, Signal::SIGCHLD as i32)
            });

            let mut sig = MaybeUninit::zeroed().assume_init();
            assert_eq!(chld_business, "chld_business");
            assert_eq!(hup_business, "hup_business");
            signal_handler(libc::SIGHUP, &mut sig, null_mut());
            assert_eq!(hup_business, "hup received");
            assert_eq!(chld_business, "chld_business");
            signal_handler(libc::SIGCHLD, &mut sig, null_mut());
            assert_eq!(hup_business, "hup received");
            assert_eq!(chld_business, "child received");
        }
    }
}

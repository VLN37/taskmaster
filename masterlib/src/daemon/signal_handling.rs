use std::mem::MaybeUninit;
use std::ptr::null_mut;

use libc::{c_int, sigaction, sigemptyset, SA_SIGINFO};

use super::config::Signal;

// static mut SIGHUP_CLOSURE: Option<Box<dyn FnMut(c_int)>> = None;
static mut SIGHUP_CLOSURE: Option<Box<dyn FnMut()>> = None;

extern "C" fn sighup_handler(_sig: c_int) {
    unsafe {
        if let Some(ref mut handler) = SIGHUP_CLOSURE {
            println!("calling closure");
            handler();
            println!("closure called");
        }
    }
}

pub fn install_sighup_handler(handler: impl FnMut() + 'static) {
    let mut action: sigaction = unsafe { MaybeUninit::zeroed().assume_init() };

    unsafe { SIGHUP_CLOSURE = Some(Box::new(handler)) };
    action.sa_sigaction = sighup_handler as usize;
    action.sa_flags = SA_SIGINFO;
    unsafe { sigemptyset(&mut action.sa_mask) };

    unsafe { sigaction(Signal::SIGHUP as i32, &action, null_mut::<sigaction>()) };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::daemon::taskmaster::{Status, TaskMaster};

    #[test]
    fn handler_test() {
        let mut taskmaster = TaskMaster::new();
        let ptr: *mut Status = &mut taskmaster.status;
        unsafe {
            install_sighup_handler(move || {
                *ptr = Status::Reloading;
            });
        }
        taskmaster.status = Status::Active;
        assert_eq!(taskmaster.status, Status::Starting);
        sighup_handler(libc::SIGHUP);
        assert_eq!(taskmaster.status, Status::Reloading);
        taskmaster.status = Status::Active;
        sighup_handler(libc::SIGHUP);
        assert_eq!(taskmaster.status, Status::Reloading);
        taskmaster.status = Status::Active;
        sighup_handler(libc::SIGHUP);
        assert_eq!(taskmaster.status, Status::Reloading);
        taskmaster.status = Status::Active;
    }
}

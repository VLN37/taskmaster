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

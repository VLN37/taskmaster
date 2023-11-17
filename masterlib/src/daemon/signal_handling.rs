use std::{
    mem::MaybeUninit,
    sync::mpsc::{self, Receiver},
};

use libc::{c_int, sigaction, sigemptyset, SA_SIGINFO};

use super::config::Signal;

// static mut SIGHUP_CLOSURE: Option<Box<dyn FnMut(c_int)>> = None;
static mut SIGHUP_CLOSURE: Option<Box<dyn FnOnce(c_int)>> = None;

extern "C" fn sighup_handler(sig: c_int) {
    unsafe {
        SIGHUP_CLOSURE.take().unwrap()(sig);
        // if let Some(ref mut closure) = SIGHUP_CLOSURE {
        //     println!("calling closure");
        //     closure(sig);
        //     println!("closure called");
        // }
    }
}

pub fn install_sighup_handler() -> Receiver<i32> {
    let mut action: sigaction = unsafe { MaybeUninit::zeroed().assume_init() };
    let (tx, rx) = mpsc::channel::<i32>();
    let handler = move |sig| {
        tx.send(sig).unwrap();
    };

    unsafe { SIGHUP_CLOSURE = Some(Box::new(handler)) };
    action.sa_sigaction = sighup_handler as usize;
    action.sa_flags = SA_SIGINFO;
    unsafe { sigemptyset(&mut action.sa_mask) };

    unsafe {
        sigaction(
            Signal::SIGHUP as i32,
            &action,
            std::ptr::null_mut::<sigaction>(),
        )
    };

    rx
}

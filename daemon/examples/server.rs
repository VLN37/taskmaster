use std::io::Read;
use std::mem::MaybeUninit;
use std::os::unix::net::UnixListener;
use std::path::Path;

// use libc::{sigaction, sigemptyset, siginfo_t, SA_SIGINFO};

use masterlib::daemon::config::structs::{
    sigaction, sigemptyset, siginfo_t, SA_SIGINFO,
};
use masterlib::daemon::config::Signal;

// use crate::mini_sigaction::{sigaction, sigemptyset, siginfo_t, SA_SIGINFO};

fn handler(sig: i32, info: siginfo_t) {
    let pid = unsafe { info.si_pid() };
    panic!("signal caught: {sig}, pid: {:?}", pid);
}

mod mini_sigaction {}

// cargo run -p daemon --example server
fn main() {
    let socket = Path::new(masterlib::SOCKET_PATH);
    if socket.exists() {
        std::fs::remove_file(socket).unwrap();
        println!("previous socket removed")
    }
    let listener = match UnixListener::bind(socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };
    println!("server started");
    println!("pid: {}", std::process::id());

    let mut action: sigaction = unsafe { MaybeUninit::zeroed().assume_init() };
    action.sa_sigaction = handler as usize;
    action.sa_flags = SA_SIGINFO;
    unsafe { sigemptyset(&mut action.sa_mask) };
    let mut old_action: sigaction = unsafe { MaybeUninit::zeroed().assume_init() };

    unsafe { sigaction(2, &action, &mut old_action) };
    unsafe { sigaction(Signal::SIGHUP as i32, &action, &mut old_action) };

    for client in listener.incoming() {
        let mut buffer = String::new();
        client.unwrap().read_to_string(&mut buffer).unwrap();
        println!("Client: {}", buffer);
    }
}

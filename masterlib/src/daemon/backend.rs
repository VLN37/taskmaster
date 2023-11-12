use std::{
    error::Error,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
};

pub struct BackEnd {
    pub socket: UnixListener,
}

impl BackEnd {
    pub fn new() -> BackEnd {
        BackEnd::default()
    }

    pub fn accept(&self) {
        match self.socket.accept() {
            Ok(client) => client,
            Err(e) => panic!("{e:?}"),
        };
    }

    pub fn process(&self, client: &mut UnixStream) -> Result<(), Box<dyn Error>> {
        let mut buf = String::new();
        client.read_to_string(&mut buf)?;
        println!("client: |{buf}|");
        client.write_all(b"message received")?;
        Ok(())
    }
}

impl Default for BackEnd {
    fn default() -> BackEnd {
        if Path::new(crate::SOCKET_PATH).exists() {
            std::fs::remove_file(crate::SOCKET_PATH).unwrap();
            println!("previous socket removed");
        }
        let socket = match UnixListener::bind(crate::SOCKET_PATH) {
            Ok(val) => val,
            Err(e) => panic!("{e:?}"),
        };
        BackEnd { socket }
    }
}

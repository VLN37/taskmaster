use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use super::server::Key;

#[derive(Default)]
pub struct BackEnd {
    pub clients: HashMap<u64, UnixStream>,
}

impl BackEnd {
    pub fn new() -> BackEnd {
        BackEnd::default()
    }

    pub fn recv(&mut self, key: Key) -> Result<(), Box<dyn Error>> {
        let mut buf = String::new();
        if let Some(client) = self.clients.get_mut(&key) {
            client.read_to_string(&mut buf)?;
            println!("#{key} RECEIVED: |{buf}|");
        } else {
            println!("backend: invalid key {key}");
        }
        Ok(())
    }

    pub fn send(&mut self, key: Key) -> Result<(), Box<dyn Error>> {
        if let Some(client) = self.clients.get_mut(&key) {
            client.write_all(b"message received")?;
            client.shutdown(std::net::Shutdown::Both)?;
        } else {
            println!("backend: invalid key {key}");
        }
        Ok(())
    }
}

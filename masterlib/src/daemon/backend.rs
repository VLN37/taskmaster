use std::{
    error::Error,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

#[derive(Default)]
pub struct BackEnd {}

impl BackEnd {
    pub fn new() -> BackEnd {
        BackEnd::default()
    }

    pub fn process(&self, client: &mut UnixStream) -> Result<(), Box<dyn Error>> {
        let mut buf = String::new();
        client.read_to_string(&mut buf)?;
        println!("client: |{buf}|");
        client.write_all(b"message received")?;
        Ok(())
    }
}

use std::error::Error;

use ctl::Client;

fn main() -> Result<(), Box<dyn Error>> {
    println!("hello client");
    let mut client = Client::new();
    client.build()?;
    loop {
        client.serve_routine()?;
    }
}

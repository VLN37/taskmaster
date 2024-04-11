use std::error::Error;

use ctl::Client;
// use common::DFL_SERVER_SOCKET_PATH;
// use ctl::client::client_class::Client;

fn main() -> Result<(), Box<dyn Error>> {
    println!("hello client");
    let mut client = Client::new();
    client.build()?;
    loop {
        client.serve_routine()?;
    }
    // Ok(())
}

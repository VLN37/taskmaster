use masterlib::daemon::BackEnd;

fn main() {
    let backend = BackEnd::new();
    println!("Awaiting front-end connection");
    backend.accept();
    for conn in backend.socket.incoming() {
        println!("CONNECTED");
        let mut client = match conn {
            Ok(c) => c,
            Err(e) => {
                println!("connect failed {e:?}");
                continue;
            }
        };
        backend.process(&mut client).unwrap_or_else(|x| println!("err: {x:?}"));
    }
}

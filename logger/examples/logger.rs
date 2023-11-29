use logger::Logger;

fn main() {
    let log = Logger::new("main");
    log.debug("logando");
    log.info("logando");
    log.warn("logando");
    log.error("logando");
    let log = Logger::new("main");
    println!("{:?}", log.log_level);

    println!("{:?}", log);
}

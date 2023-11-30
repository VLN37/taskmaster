use logger::{debug, error, info, warning, Logger};

fn main() {
    let log = Logger::new("main");
    log.debug("logando");
    log.info("logando");
    log.warn("logando");
    log.error("logando");
    let log = Logger::new("main");
    // println!("{:?}", log.log_level);
    // println!("{:?}", log);
    debug!("{:?}", log);
    info!("#{} AWAITING", 42);
    // self.log.info(&format!("#{} AWAITING", self.server.key));
    warning!("{:?}", log);
    error!("{:?}", log);
}

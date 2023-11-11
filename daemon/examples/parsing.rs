use masterlib::daemon::config;

fn main() {
    let f = std::fs::File::open(masterlib::CONFIG_PATH).expect("Could not open file.");
    let config = config::read(f).unwrap();
    for (key, program) in &config.programs {
        println!("{key}\n{:?}\n\n", program);
    }
}

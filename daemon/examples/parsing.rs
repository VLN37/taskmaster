use masterlib::daemon::TaskMasterConfig;

fn main() {
    let f = std::fs::File::open(masterlib::CONFIG_PATH).expect("Could not open file.");
    let config = TaskMasterConfig::read(f).unwrap();
    // let config = TaskMasterConfig::from(f);
    for (key, program) in &config.programs {
        println!("{key}\n{:?}\n\n", program);
    }
}

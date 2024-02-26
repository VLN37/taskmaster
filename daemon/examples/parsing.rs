use daemon::defs::DFL_CONFIG_FILE;
use daemon::TaskMasterConfig;
use logger::debug;

fn main() {
    let f = std::fs::File::open(DFL_CONFIG_FILE).expect("Could not open file.");
    let config = TaskMasterConfig::read(f).unwrap();
    // let config = TaskMasterConfig::from(f);
    for (key, program) in &config.programs {
        debug!("{key}\n{:?}\n\n", program);
    }
}

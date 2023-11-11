use masterlib::daemon::config;

fn main() {
    let f = std::fs::File::open("config.yml").expect("Could not open file.");
    let config = config::read(f);
    // println!("{:?}", config);
    // println!("{:?}", config.programs);
    // println!("{:?}", config.programs["first_program"]);
    // println!("{:?}", RestartOption::from_str("ALWAYS"));
    // println!("$ {}", std::env::var("PWD").unwrap());
    // println!("{}", RestartOption::ALWAYS as i32 == 0);
    for (key, program) in &config.programs {
        println!("{key}\n{:?}\n\n", program);
    }
}

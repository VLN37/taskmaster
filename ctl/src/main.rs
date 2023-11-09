use masterlib::config;

fn main() {
    println!("Hello, world!");
    let val = config::RestartOption::ALWAYS;
    println!("{:?}", val);
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
struct Config {
  command: String,
  numbers: Vec<u32>
}

impl Default for Config {
  fn default() -> Config {
    Config {
      command: String::from("cat"),
      numbers: [1, 2, 3, 4, 5].to_vec(),
    }
  }
}

fn main() {
  let f = std::fs::File::open("config.yml").expect("Could not open file.");
  let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
  println!("Hello, world!");
  println!("{:?}", scrape_config);
  println!("{:?}", scrape_config.command);
  println!("{:?}", scrape_config.numbers);
}

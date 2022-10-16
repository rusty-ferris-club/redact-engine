use redact_engine::{Pattern, Redaction};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Config {
    patterns: Vec<Pattern>,
}

fn main() {
    let file = std::fs::File::open("./src/redact-config.yaml").unwrap();
    let config: Config = serde_yaml::from_reader(file).unwrap();
    let redact = Redaction::new().add_patterns(config.patterns);

    println!("{}", redact.redact_str("string to redact: foo,bar"));
}

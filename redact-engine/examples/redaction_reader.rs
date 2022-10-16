use anyhow::Result;
use redact_engine::{Pattern, Redaction};
use regex::Regex;

fn main() -> Result<()> {
    let file = std::fs::File::open("redact-engine/tests/test.txt")?;

    let pattern = Pattern {
        test: Regex::new("(foo)")?,
        group: 1,
    };

    let redaction = Redaction::new().add_pattern(pattern);
    println!("{:#?}", redaction.redact_reader(file));

    Ok(())
}

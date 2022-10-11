use anyhow::Result;
use regex::Regex;
use text_redaction::{Pattern, Redaction};

fn main() -> Result<()> {
    let file = std::fs::File::open("text-redaction/tests/test.txt")?;

    let pattern = Pattern {
        test: Regex::new("(foo)")?,
        group: 1,
    };

    let redaction = Redaction::new().add_pattern(pattern);
    println!("{:#?}", redaction.redact_reader(file));

    Ok(())
}

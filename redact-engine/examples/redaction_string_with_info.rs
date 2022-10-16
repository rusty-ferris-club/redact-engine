use anyhow::Result;
use redact_engine::{Pattern, Redaction};
use regex::Regex;

fn main() -> Result<()> {
    let text = "foo,bar";

    let pattern = Pattern {
        test: Regex::new("(bar)")?,
        group: 1,
    };

    let redaction = Redaction::new().add_pattern(pattern);
    println!("{:#?}", redaction.redact_str_with_info(text));

    Ok(())
}

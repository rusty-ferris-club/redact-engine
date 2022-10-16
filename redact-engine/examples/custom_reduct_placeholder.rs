use anyhow::Result;
use redact_engine::{Pattern, Redaction};
use regex::Regex;

fn main() -> Result<()> {
    let text = "foo,bar";

    let pattern = Pattern {
        test: Regex::new("(bar)")?,
        group: 1,
    };

    let redact = Redaction::custom("[HIDDEN_TEXT]").add_pattern(pattern);
    println!("{:#?}", redact.redact_str(text));

    Ok(())
}

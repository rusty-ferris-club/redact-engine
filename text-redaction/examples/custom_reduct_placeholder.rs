use anyhow::Result;
use regex::Regex;
use text_redaction::{Pattern, Redaction};

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

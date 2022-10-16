use anyhow::Result;
use redact_engine::Redaction;

fn main() -> Result<()> {
    let text = "foo,bar";

    let redaction = Redaction::new().add_value("foo")?;
    println!("{:#?}", redaction.redact_str(text));

    Ok(())
}

use anyhow::Result;
use text_redaction::Redaction;

fn main() -> Result<()> {
    let text = "foo,bar";

    let redaction = Redaction::new().add_value("foo")?;
    println!("{:#?}", redaction.redact_str(text));

    Ok(())
}

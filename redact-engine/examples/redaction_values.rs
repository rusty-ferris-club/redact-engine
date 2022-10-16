use anyhow::Result;
use redact_engine::Redaction;

fn main() -> Result<()> {
    let text = "foo,bar,baz";

    let redaction = Redaction::new().add_values(vec!["foo", "bar"])?;
    println!("{:#?}", redaction.redact_str(text));

    Ok(())
}

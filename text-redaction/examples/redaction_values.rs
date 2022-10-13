use anyhow::Result;
use text_redaction::Redaction;

fn main() -> Result<()> {
    let text = "foo,bar,baz";

    let redaction = Redaction::new().add_values(vec!["foo", "bar"])?;
    println!("{:#?}", redaction.redact_str(text));

    Ok(())
}

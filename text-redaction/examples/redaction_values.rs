use anyhow::Result;
use text_redaction::Redaction;

fn main() -> Result<()> {
    let text = "foo,bar,ba',?^z";

    let redaction = Redaction::new().add_values(vec!["foo", "ba',?^z"])?;
    println!("{:#?}", redaction.redact_str(text));

    Ok(())
}

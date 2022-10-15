#[cfg(feature = "redact-json")]
use serde_json::json;
use text_redaction::Redaction;

fn main() {
    let redaction = Redaction::new().add_keys(vec!["bar", "array"]);

    let json = json!({
        "key": "value",
        "array": ["val-1", "val-2"],
        "foo": {
            "bar": true,
            "baz": "bar"
        },
        "a": {
            "b": {
                "c": "value"
            }
        }
    })
    .to_string();

    println!("{:#?}", redaction.redact_json(&json));
}

#[cfg(feature = "redact-json")]
use serde_json::json;
use text_redaction::Redaction;

fn main() {
    let redaction = Redaction::new().add_paths(vec!["a.b.*", "a.foo"]);

    let json = json!({
    "a": {
        "b": {
            "key": "redact_me",
            "key2": "redact_me",
        },
        "foo": "bar",
        "key": "skip-redaction"
    },
    "key": "skip-redaction"
    })
    .to_string();

    println!("{:#?}", redaction.redact_json(&json));
}

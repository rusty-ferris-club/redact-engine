use serde_json::json;
use text_redaction::Redaction;

fn main() {
    let redact = Redaction::default()
        .add_paths(vec!["foo.*", "bar.key"])
        .add_keys(vec!["key"]);

    let json = json!({
    "foo": {
        "b": {
            "key": "redact_me",
        },
        "foo": "redact_me",
        "key": "redact_me",
    },
    "bar": {
        "b": {
            "key": "skip-redaction",
        },
        "foo": "skip-redaction",
        "key": "redact_me"
    },
    "key": "redact_me",
    "baz": "skip-redaction"
    })
    .to_string();

    println!("{:#?}", redact.redact_json(&json));
}

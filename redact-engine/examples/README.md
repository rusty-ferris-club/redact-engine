# redact-engine examples

Here you can find all redaction seniors that we can offer
 - [Change redact placeholder text.](./custom_reduct_placeholder.rs)
 - [From io::read.](./redaction_reader.rs)
 
 - [Using regex pattern.](./redaction_string.rs)
 - [Provide a single string.](./redaction_json_by_keys.rs)
 - [Provide multiple strings.](./redaction_values.rs)

JSON format - `redact-json` feature flag should be enabled
 - [Multiple JSON keys.](./redaction_json_by_keys.rs)
 - [Providing JSON path.](./redaction_json_by_path.rs)
 - [From Value.](./redaction_json_value.rs)
 - [Mix rules.](./redaction_json.rs)

Redact capture information - `redact-info` feature flag should be enabled
 - [Return redact matches information.](./redaction_string_with_info.rs)
## Run

```bash
cargo run --example [example-name]
```
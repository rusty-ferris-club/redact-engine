# text-redaction

Redact text

```rs
use text_redaction::{Pattern, Redaction};

let text = "foo,bar";

let pattern = Pattern {
    test: Regex::new("(bar)")?,
    group: 1,
};

let redaction = Redaction::new().add_pattern(pattern);
let redacted_string = redaction.redact_str(text);
```

## Usage
Add this to Cargo.toml:

[dependencies]
text-redaction = { version = "0.1.0" }

## Formats

 - Text
 - io::Read
 - JSON - should enable `redact-json` feature flag

## Example

### from string 
```sh
cargo run --example redaction_string
```
### from reader 
```sh
cargo run --example redaction_reader
```

To see more code example, run the command `cargo run --example`
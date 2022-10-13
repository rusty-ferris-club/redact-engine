# text-redaction

## Usage
Add this to Cargo.toml:
```toml
[dependencies]
text-redaction = { version = "0.1.0" }
```

## Usecase
 - Redact sensitive information from a text text file
 - Redact JSON schema by providing
    - By key
    - Prefix path
 - Integrate with [env_logger](./text-redaction//examples/logger/env-logger)


### Redact by specific value 

```rs
use text_redaction::{Pattern, Redaction};

let text = "foo,bar";

let redaction = Redaction::new().add_value("foo")?;
let redacted_string = redaction.redact_str(text);
```


## Example

To see all code [example](./text-redaction/examples), run the command `cargo run --example`

### Redact from regex pattern

```rs
use text_redaction::{Pattern, Redaction};

let text = "foo,bar";

let pattern = Pattern {
    test: Regex::new("(bar)").unwrap(),
    group: 1,
};

let redaction = Redaction::new().add_pattern(pattern);
let redacted_string = redaction.redact_str(text);
```

### Redact with mix
```rs
use text_redaction::{Pattern, Redaction};

let text = "foo,bar,baz,extra";

let pattern = vec![
    Pattern {test: Regex::new("(bar)").unwrap(),group: 1},
    Pattern {test: Regex::new("(bar)").unwrap(),group: 1},
];
let redaction = Redaction::new().add_patterns(pattern).add_values(vec!["baz", "extra"]);
let redacted_string = redaction.redact_str(text);
```

### Redact from JSON
```rs
let json = json!({
    "all-path": {
        "b": {
            "key": "redact_me",
        },
        "foo": "redact_me",
        "key": "redact_me",
    },
    "specific-key": {
        "b": {
            "key": "skip-redaction",
        },
        "foo": "skip-redaction",
        "key": "redact_me"
    },
    "key": "redact_me",
    "skip": "skip-redaction",
    "by-value": "bar",
    "by-pattern": "redact-by-pattern",
})
.to_string();

let redaction = Redaction::default()
    .add_pattern(pattern)
    .add_path("all-path.*")
    .add_path("specific-key.key")
    .add_key("key")
    .add_value("bar");
let redacted_json = redaction.redact_json(&json)
```

# Contributing

We are accepting PRs. Feel free to [submit PRs](https://github.com/rusty-ferris-club/text-redaction/pulls).

To all [Contributors](https://github.com/rusty-ferris-club/text-redaction/graphs/contributors) - you make this happen, thanks!

[contributing guide](CONTRIBUTING.md)

# Copyright

Copyright (c) 2022 [@kaplanelad](https://github.com/kaplanelad). See [LICENSE](LICENSE) for further details.

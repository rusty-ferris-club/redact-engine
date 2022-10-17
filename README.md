# redact-engine
[![Crates.io](https://img.shields.io/crates/v/redact-engine?style=flat-square)](https://crates.io/crates/redact-engine)
[![CI](https://github.com/rusty-ferris-club/redact-engine/actions/workflows/ci.yaml/badge.svg)](https://github.com/rusty-ferris-club/redact-engine/actions/workflows/ci.yaml)

Protect confidentiality with dynamic redaction by replacing sensitive data from string or JSON format
[API Docs](https://docs.rs/redact-engine/0.1.0/redact_engine)
## Usage
Add this to Cargo.toml:
```toml
[dependencies]
redact-engine = { version = "0.1.2" }
```

## Usecase
 - Redact sensitive information from a text text file
 - Redact JSON schema by providing
    - By key
    - Prefix path
 - Redact logs (supporting [env_logger](./redact-engine//examples/logger/env-logger))

## Example:
```rs
use text_redaction::{Pattern, Redaction};

let text = "some string message that you want to redact: foo,bar";

let redaction = Redaction::new().add_value("foo")?;
let redacted_string = redaction.redact_str(text);

// println!("{}", redacted_string); 
// some string message that you want to redact: [TEXT_REDACTED],bar
```

To see all code [example](./redact-engine/examples), run the command `cargo run --example`

## Supported features 
 - `redact-json` - Redact from JSON format
 - `redact-info` - Return redact capture information (position and pattern ID)

# Benchmark test

Redact type | function | Times | Size | Results
--- | ---| --- | --- | --- 
Text | add_patterns | 1,000 | 70 chars | 29.016 µs |
Text | add_values | 1,000 | 70 chars | 27.881s µs |
Text | redact_reader | 1,000 | 70 chars | 117.75 µs |
Text | redact_str_with_info | 1,000 | 70 chars | 36.532 µs |
JSON | add_keys | 1,000 | 15 keys | 8.5483 µs |
JSON | add_paths | 1,000 | 15 keys | 7.0353 µs |
JSON | from_value | 1,000 | 15 keys | 8.7555 µs |

# Contributing

We are accepting PRs. Feel free to [submit PRs](https://github.com/rusty-ferris-club/redact-engine/pulls).

To all [Contributors](https://github.com/rusty-ferris-club/redact-engine/graphs/contributors) - you make this happen, thanks!

[contributing guide](CONTRIBUTING.md)

# Copyright

Copyright (c) 2022 [@kaplanelad](https://github.com/kaplanelad). See [LICENSE](LICENSE) for further details.

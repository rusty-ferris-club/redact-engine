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

To see all code [example](./text-redaction/examples), run the command `cargo run --example`


# Benchmark test

Redact type | function | Times | Size | Results
--- | ---| --- | --- | --- 
Text | add_patterns | 1,000 | 70 chars | 29.016 µs |
Text | add_values | 1,000 | 70 chars | 27.881s µs |
Text | redact_reader | 1,000 | 70 chars | 117.75 µs |
Text | redact_str_with_info | 1,000 | 70 chars | 36.532 µs |
JSON | add_keys | 1,000 | 15 keys | 8.5483 µs |
JSON | add_paths | 1,000 | 15 keys | 7.0353 µs |

# Contributing

We are accepting PRs. Feel free to [submit PRs](https://github.com/rusty-ferris-club/text-redaction/pulls).

To all [Contributors](https://github.com/rusty-ferris-club/text-redaction/graphs/contributors) - you make this happen, thanks!

[contributing guide](CONTRIBUTING.md)

# Copyright

Copyright (c) 2022 [@kaplanelad](https://github.com/kaplanelad). See [LICENSE](LICENSE) for further details.

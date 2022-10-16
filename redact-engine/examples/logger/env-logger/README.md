# Redact env_logger

[redact-engine](https://github.com/rusty-ferris-club/redact-engine) can redact log messages by the following example:


```rs
fn main() {
    let env = Env::new().filter_or("MY_LOG_LEVEL", "trace");

    let remove_foo_pattern = Pattern {
        test: Regex::new("(bar)").unwrap(),
        group: 1,
    };
    let redaction = Redaction::new().add_pattern(remove_foo_pattern);

    Builder::from_env(env)
        .format(move |buf, record| {
            let timestamp = buf.timestamp();
            writeln!(
                buf,
                "[{} {} {}] {}",
                timestamp,
                record.level(),
                record.module_path().unwrap_or_default(),
                redaction.redact_str(&record.args().to_string())
            )
        })
        .init();

    log::info!("log message ");
    log::info!("message that include bar");
}

[2022-10-10T17:21:02Z INFO redact_env_logger] log message
[2022-10-10T17:21:02Z INFO redact_env_logger] message that include [TEXT_REDACTED]
```

## Run Example:
```bash
cargo run 
```
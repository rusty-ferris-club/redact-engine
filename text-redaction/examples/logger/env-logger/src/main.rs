use std::io::Write;

use env_logger::{Builder, Env};
use regex::Regex;
use text_redaction::{Pattern, Redaction};

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

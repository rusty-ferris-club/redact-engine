use std::{env, fs::File, io::Write};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use redact_engine::{Pattern, Redaction};
use regex::Regex;

mod utils;

const TEXT: &str = "redact example benchmarking test with the detections text: foo,bar,baz";

fn get_redact_patterns(count: usize) -> Redaction {
    let mut patterns = vec![
        Pattern {
            test: Regex::new("(foo)").unwrap(),
            group: 1,
        },
        Pattern {
            test: Regex::new("(bar)").unwrap(),
            group: 1,
        },
        Pattern {
            test: Regex::new("(baz)").unwrap(),
            group: 1,
        },
    ];

    for _ in 0..count - patterns.len() {
        patterns.push(Pattern {
            test: Regex::new(&format!("({})", utils::get_random_string(10))).unwrap(),
            group: 1,
        });
    }

    Redaction::new().add_patterns(patterns)
}

fn get_redact_values(count: usize) -> Redaction {
    let mut values = vec!["foo", "bar", "baz"];
    let mut random_values = vec![];
    for _ in 0..count - values.len() {
        random_values.push(utils::get_random_string(10));
    }
    random_values.iter().for_each(|s| values.push(s.as_str()));
    Redaction::new().add_values(values).unwrap()
}

fn redact_str_benchmark(c: &mut Criterion) {
    let file_path = env::temp_dir().join("bench_test.txt");
    let mut f = File::create(&file_path).unwrap();
    #[allow(clippy::unused_io_amount)]
    f.write(TEXT.as_bytes()).unwrap();

    let redaction_with_patterns = get_redact_patterns(100);
    let redaction_with_values = get_redact_values(100);

    let mut banch_group = c.benchmark_group("redact_str");
    banch_group.sample_size(1_0000);

    banch_group.bench_function("redact_str_from_patterns", |b| {
        b.iter(|| redaction_with_patterns.redact_str(black_box(TEXT)));
    });

    banch_group.bench_function("redact_str_from_values", |b| {
        b.iter(|| redaction_with_values.redact_str(black_box(TEXT)));
    });

    banch_group.bench_function("redact_str_from_reader", |b| {
        b.iter(|| redaction_with_values.redact_reader(black_box(File::open(&file_path).unwrap())));
    });

    banch_group.bench_function("redact_str_with_info", |b| {
        b.iter(|| redaction_with_values.redact_str_with_info(black_box(TEXT)));
    });
    banch_group.finish();
}

criterion_group!(benches, redact_str_benchmark);
criterion_main!(benches);

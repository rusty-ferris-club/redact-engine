use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regex::Regex;
use text_redaction::{Pattern, Redaction};

mod utils;

const JSON: &str = r#"
 {
   "all-path":{
      "b":{
         "key":"redact_me"
      },
      "foo":"redact_me",
      "key":"redact_me"
   },
   "specific-key":{
      "b":{
         "key":"skip-redaction"
      },
      "foo":"skip-redaction",
      "key":"redact_me"
   },
   "key":"redact_me",
   "skip":"skip-redaction",
   "by-value":"bar",
   "by-pattern":"redact-by-pattern",
   "sample": "value",
}
"#;

fn get_redact_keys(count: usize) -> Redaction {
    let mut keys = vec!["bar", "by-pattern"];
    let mut random_values = vec![];
    for _ in 0..count - keys.len() {
        random_values.push(utils::get_random_string(10));
    }
    random_values.iter().for_each(|s| keys.push(s.as_str()));
    Redaction::new().add_keys(keys)
}

fn get_redact_path(count: usize) -> Redaction {
    let mut paths = vec!["all-path.*", "specific-key.key", "key"];
    let mut random_paths = vec![];
    for _ in 0..count - paths.len() {
        random_paths.push(utils::get_random_string(10));
    }
    random_paths.iter().for_each(|s| paths.push(s.as_str()));
    Redaction::new().add_paths(paths)
}

fn redact_json_benchmark(c: &mut Criterion) {
    let mut banch_group = c.benchmark_group("redact_json");
    banch_group.sample_size(1_000);

    let patterns = vec![
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

    let redaction_with_keys = get_redact_keys(100);
    let redaction_with_path = get_redact_path(100);

    banch_group.bench_function("redact_json_from_keys", |b| {
        b.iter(|| redaction_with_keys.redact_json(black_box(JSON)));
    });

    banch_group.bench_function("redact_json_from_path", |b| {
        b.iter(|| redaction_with_path.redact_json(black_box(JSON)));
    });

    banch_group.finish();
}

criterion_group!(benches, redact_json_benchmark);
criterion_main!(benches);

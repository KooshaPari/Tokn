use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_parse_json(c: &mut Criterion) {
    let sample = r#"{"model":"gpt-4","prompt_tokens":120,"completion_tokens":80,"cost_usd":0.012}"#;
    c.bench_function("parse_json", |b| {
        b.iter(|| {
            let v: serde_json::Value = serde_json::from_str(black_box(sample)).unwrap();
            black_box(v);
        })
    });
}

fn bench_format_display(c: &mut Criterion) {
    c.bench_function("format_display", |b| {
        b.iter(|| {
            let formatted = format!("{:.6}", black_box(0.0123456789));
            black_box(formatted);
        })
    });
}

criterion_group!(benches, bench_parse_json, bench_format_display);
criterion_main!(benches);

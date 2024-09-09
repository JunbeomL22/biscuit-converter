use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_parser::BiscuitParser;
use atoi::atoi;

fn bench_i64(c: &mut Criterion) {
    let biscuit_parser = BiscuitParser::default();
    let test_set = vec![
        "-123", 
        "-123456",
        "-123456789",
        "-123456789012",
        "-123456789012345",
        "-123456789012345678",
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(input_str);
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| biscuit_parser.to_i64(black_box(input))));
        group.bench_function("std", |b| b.iter(|| black_box(input_str).parse::<i64>().unwrap()));
        group.bench_function("atoi", |b| b.iter(|| atoi::<i64>(black_box(input)).unwrap()));
        group.finish();
    }
}

criterion_group!(
    benches, 
    bench_i64,
);

criterion_main!(benches);
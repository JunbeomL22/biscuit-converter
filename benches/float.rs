use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_parser::BiscuitParser;

fn bench_f32(c: &mut Criterion) {
    let biscuit_parser = BiscuitParser::default();
    let test_set = vec![
        "1.0", "12.0", "123.0", "1234.0", 
        /*
        "12345.0", "123456.0", "1234567.0", "12345678.0", 
        "123456789.0", "1234567890.0", "12345678901.0", "123456789012.0", 
        "1234567890123.0", "12345678901234.0", "123456789012345.0", "1234567890123456.0", 
        "12345678901234567.0", "123456789012345678.0", "1234567890123456789.0", "12345678901234567890.0", 
        "123456789012345678901.0", "1234567890123456789012.0", "12345678901234567890123.0", "123456789012345678901234.0",
        "1234567890123456789012345.0", "12345678901234567890123456.0", 
        "123456789012345678901234567.0", "1234567890123456789012345678.0", 
        "12345678901234567890123456789.0", "123456789012345678901234567890.0",
        "1234567890123456789012345678901.0", "12345678901234567890123456789012.0",
         */
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(input_str);
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| biscuit_parser.to_f32(black_box(input))));
        group.bench_function("std", |b| b.iter(|| black_box(input_str).parse::<f32>().unwrap()));
    }
}

criterion_group!(
    benches, 
    bench_float,
);

criterion_main!(benches);
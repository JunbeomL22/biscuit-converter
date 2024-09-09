use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::BiscuitConverter;

fn bench_f32(c: &mut Criterion) {
    let biscuit_converter = BiscuitConverter::initialize();
    let biscuit_converter_fraction_given = BiscuitConverter::initialize().with_fraction_length(2);

    let test_set = vec![
        "1.23", 
        "1234.56",
        "1234567.89",
        "1234567890.12",
        "1234567890123.45",
        "1234567890123456.78",
        "1234567890123456789.01",
        "1234567890123456789012.34",
        "1234567890123456789012345.67",
        "1234567890123456789012345678.90",
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(input_str);
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| biscuit_converter.to_f64(black_box(input))));
        group.bench_function("biscuit_fraction_given", |b| b.iter(|| biscuit_converter_fraction_given.to_f64(black_box(input))));
        group.bench_function("std", |b| b.iter(|| black_box(input_str).parse::<f64>().unwrap()));
        group.finish();
    }
}

criterion_group!(
    benches, 
    bench_f32,
);

criterion_main!(benches);
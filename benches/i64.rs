use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::BiscuitConverter;
use atoi::atoi;
use atoi::FromRadix10Signed;

fn bench_i64(c: &mut Criterion) {
    let biscuit_converter = BiscuitConverter::default();
    let test_set = vec![
        "1",
        "12",
        "123",
        "1234", 
        "12345",
        "123456",
        "1234567",
        "12345678",
        "123456789",
        "1234567890",
        "123456789012",
        "1234567890123",
        "12345678901234",
        "123456789012345",
        "1234567890123456",
        "12345678901234567",
        "123456789012345678",
        "1234567890123456789",
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(format!("i64 : {}", input_str));
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| biscuit_converter.to_i64_decimal(black_box(input))));
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
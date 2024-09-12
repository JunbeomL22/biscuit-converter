use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::BiscuitConverter;
use atoi::atoi;

fn bench_i8(c: &mut Criterion) {
    let biscuit_converter = BiscuitConverter::default();
    let test_set = vec![
        "1",
        "12",
        "123",
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(format!("i8 {}", input_str).as_str());
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| biscuit_converter.to_i8_decimal(black_box(input)).unwrap()));
        group.bench_function("std", |b| b.iter(|| black_box(input_str).parse::<i8>().unwrap()));
        group.bench_function("atoi", |b| b.iter(|| atoi::<i8>(black_box(input)).unwrap()));
        group.finish();
    }
}

criterion_group!(
    benches, 
    bench_i8,
);

criterion_main!(benches);
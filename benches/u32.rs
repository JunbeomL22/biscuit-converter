use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::BiscuitConverter;
use atoi::atoi;

fn bench_u32(c: &mut Criterion) {
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
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(format!("u32 {}", input_str).as_str());
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| biscuit_converter.to_u32_decimal(black_box(input)).unwrap()));
        group.bench_function("std", |b| b.iter(|| black_box(input_str).parse::<u32>().unwrap()));
        group.bench_function("atoi", |b| b.iter(|| atoi::<u32>(black_box(input)).unwrap()));
        group.finish();
    }
}

criterion_group!(
    benches, 
    bench_u32,
);

criterion_main!(benches);
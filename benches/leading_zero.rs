use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::Biscuit;
use atoi::atoi;

fn bench_leading_zeros_u64(c: &mut Criterion) {
    let test_set = vec![
        "0000000000000000000",
        "0000000000000000001",
        "0000000000000000012",
        "0000000000000000123",
        "0000000000000001234",
        "0000000000000012345",
        "0000000000000123456",
        "0000000000001234567",
        "0000000000012345678",
        "0000000000123456789",
        "0000000001234567890",
        "0000000012345678901",
        "0000000123456789012",
        "0000001234567890123",
        "0000012345678901234",
        "0000123456789012345",
        "0001234567890123456",
        "0012345678901234567",
        "0123456789012345678",
        "1234567890123456789",
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(format!("u64 : {}", input_str));
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| u64::parse_decimal(black_box(input)).unwrap()));
        group.bench_function("std", |b| b.iter(|| black_box(input_str).parse::<u64>().unwrap()));
        group.bench_function("atoi", |b| b.iter(|| atoi::<u64>(black_box(input)).unwrap()));
        group.finish();
    }

}

criterion_group!(
    benches, 
    bench_leading_zeros_u64,
);

criterion_main!(benches);
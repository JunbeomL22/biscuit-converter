use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::BiscuitConverter;
use atoi::atoi;
use atoi::FromRadix10Signed;

fn bench_i128(c: &mut Criterion) {
    let bis = BiscuitConverter::default();
    let test_set = vec![
        "-123456789012345678901234567890123",
        "-1234567890123456789012345678901234",
        "-12345678901234567890123456789012345",
        "-123456789012345678901234567890123456",
        "-1234567890123456789012345678901234567",
        "-12345678901234567890123456789012345678",
        "-123456789012345678901234567890123456789",
        "-1",
        "-12",
        "-123", 
        "-1234",
        "-12345",
        "-123456",
        "-1234567",
        "-12345678",
        "-123456789",
        "-1234567890",
        "-12345678901",
        "-123456789012",
        "-1234567890123",
        "-12345678901234",
        "-123456789012345",
        "-1234567890123456",
        "-12345678901234567",
        "-123456789012345678",
        "-1234567890123456789",
        "-12345678901234567890",
        "-123456789012345678901",
        "-1234567890123456789012",
        "-12345678901234567890123",
        "-123456789012345678901234",
        "-1234567890123456789012345",
        "-12345678901234567890123456",
        "-123456789012345678901234567",
        "-1234567890123456789012345678",
        "-12345678901234567890123456789",
        "-123456789012345678901234567890",
        "-1234567890123456789012345678901",
        "-12345678901234567890123456789012",
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(format!("i128 {}", input_str).as_str());
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| bis.to_i128_decimal(black_box(input)).unwrap()));
        group.bench_function("std", |b| b.iter(|| black_box(input_str).parse::<i128>().unwrap()));
        group.bench_function("atoi", |b| b.iter(|| i128::from_radix_10_signed(input).0));
        group.finish();
    }
}

criterion_group!(
    benches, 
    bench_i128,
);
criterion_main!(benches);
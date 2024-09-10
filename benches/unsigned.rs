use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::BiscuitConverter;
use atoi::atoi;

fn bench_u64(c: &mut Criterion) {
    let bis = BiscuitConverter::default();
    let test_set = vec![
        "123", 
        "123456",
        "123456789",
        "123456789012",
        "123456789012345",
        "123456789012345678",
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(input_str);
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| bis.to_u128(black_box(input)).unwrap()));
        group.bench_function("std", |b| b.iter(|| black_box(input_str).parse::<u128>().unwrap()));
        group.bench_function("atoi", |b| b.iter(|| atoi::<u128>(black_box(input)).unwrap()));
        group.finish();
    }
}

criterion_group!(
    benches, 
    bench_u64,
);
criterion_main!(benches);
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::Biscuit;
use atoi::atoi;

fn bench_i16(c: &mut Criterion) {

    let test_set = vec![
        "1",
        "12",
        "123",
        "1234",
        "12345",
    ];

    for input_str in test_set {
        let mut group = c.benchmark_group(format!("i16 {}", input_str).as_str());
        let input = input_str.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| i16::parse_decimal(black_box(input)).unwrap()));
        group.bench_function("std", |b| b.iter(|| black_box(input_str).parse::<i16>().unwrap()));
        group.bench_function("atoi", |b| b.iter(|| atoi::<i16>(black_box(input)).unwrap()));
        group.finish();
    }
}

criterion_group!(
    benches, 
    bench_i16,
);

criterion_main!(benches);
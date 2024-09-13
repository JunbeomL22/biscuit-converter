use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::Biscuit;
use atoi::atoi;

fn bench_u16(c: &mut Criterion) {

    let test_set = vec![
        "1",
        "12",
        "123",
        "1234",
        "12345",
    ];

    let mut group = c.benchmark_group(format!("u16 {}", "1234"));
    
    let input = test_set[0];
    let input_bytes = input.as_bytes();
    group.bench_function("biscuit", |b| b.iter(|| u16::parse_decimal(black_box(input_bytes)).unwrap()));
    group.bench_function("std", |b| b.iter(|| black_box(input).parse::<u16>().unwrap()));
    group.bench_function("atoi", |b| b.iter(|| atoi::<u16>(black_box(input_bytes)).unwrap()));

    group.finish();
}

criterion_group!(
    benches, 
    bench_u16,
);

criterion_main!(benches);
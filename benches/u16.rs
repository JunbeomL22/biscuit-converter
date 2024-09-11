use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::BiscuitConverter;
use atoi::atoi;

fn bench_i16(c: &mut Criterion) {
    let biscuit_converter = BiscuitConverter::default();
    let test_set = vec![
        "1234",
    ];

    let mut group = c.benchmark_group(format!("u16 {}", "1234"));
    
    let input = test_set[0];
    let input_bytes = input.as_bytes();
    group.bench_function("biscuit", |b| b.iter(|| biscuit_converter.to_u16_decimal(black_box(input_bytes)).expect("to_u16 failed")));
    group.bench_function("biscuit_unchecked", |b| b.iter(|| unsafe {biscuit_converter.to_u16_unchecked(black_box(input))}));
    group.bench_function("std", |b| b.iter(|| black_box(input).parse::<i16>().unwrap()));
    group.bench_function("atoi", |b| b.iter(|| atoi::<i16>(black_box(input_bytes)).unwrap()));

    group.finish();
}

criterion_group!(
    benches, 
    bench_i16,
);

criterion_main!(benches);
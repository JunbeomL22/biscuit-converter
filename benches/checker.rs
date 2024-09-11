use criterion::{criterion_group, criterion_main, Criterion, black_box};

use biscuit_converter::little_endian::{
    check_decimal,
    check_decimal_bit_u16,
    check_decimal_bit_u32,
    check_decimal_bit_u64,
    check_decimal_bit_u128,
    le_bytes_to_u16,
    le_bytes_to_u32,
    le_bytes_to_u64,
    le_bytes_to_u128,
};


fn check_decimal_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_decimal u16");
    let x = b"12";
    let x_chunk = le_bytes_to_u16(x);

    group.bench_function("check_decimal", |b| b.iter(|| check_decimal(black_box(x))));
    group.bench_function("check_decimal_bit_u16", |b| b.iter(|| check_decimal_bit_u16(black_box(x_chunk))));

    group.finish();

    let mut group = c.benchmark_group("check_decimal u32");
    let x = b"1234";
    let x_chunk = le_bytes_to_u32(x);

    group.bench_function("check_decimal", |b| b.iter(|| check_decimal(black_box(x))));
    group.bench_function("check_decimal_bit_u32", |b| b.iter(|| check_decimal_bit_u32(black_box(x_chunk))));

    group.finish();

    let mut group = c.benchmark_group("check_decimal u64");
    let x = b"34567890";
    let x_chunk = le_bytes_to_u64(x);
    
    group.bench_function("check_decimal", |b| b.iter(|| check_decimal(black_box(x))));
    group.bench_function("check_decimal_bit_u64", |b| b.iter(|| check_decimal_bit_u64(black_box(x_chunk))));

    group.finish();

    let mut group = c.benchmark_group("check_decimal u128");
    let x = b"1234567890123456";
    let x_chunk = le_bytes_to_u128(x);

    group.bench_function("check_decimal", |b| b.iter(|| check_decimal(black_box(x))));
    group.bench_function("check_decimal_bit_u128", |b| b.iter(|| check_decimal_bit_u128(black_box(x_chunk))));

    group.finish();
}

criterion_group!(benches, check_decimal_benchmark);
criterion_main!(benches);


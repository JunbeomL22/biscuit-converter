use criterion::{black_box, criterion_group, criterion_main, Criterion};
use atoi::atoi;
use biscuit_converter::little_endian::{
    le_bytes_to_u64,
    le_bytes_to_u128,
    sixteen_to_u128,
    eight_to_u64,
};

fn criterion_benchmark(c: &mut Criterion) {
    let x = b"12345678";
    let x8: &[u8] = &x[..];

    let x = b"1234567890123456";
    let x16: &[u8] = &x[..];

    let mut group = c.benchmark_group("base");
    group.bench_function("8 bytes", |b| b.iter(|| eight_to_u64(le_bytes_to_u64(black_box(x8)))));
    group.bench_function("16 bytes", |b| b.iter(|| sixteen_to_u128(le_bytes_to_u128(black_box(x16)))));
    group.finish();

    let mut group = c.benchmark_group("atoi");
    group.bench_function("8 bytes", |b| b.iter(|| atoi::<u64>(black_box(x8))));
    group.bench_function("16 bytes", |b| b.iter(|| atoi::<u128>(black_box(x16))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
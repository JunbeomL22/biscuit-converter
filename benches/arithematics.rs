use criterion::{criterion_group, criterion_main, Criterion, black_box};
use flashparser::arithematics::{
    u64_length,
    u32_length,
    div10u32,
    div10u64,
};

fn bench_divrem_bit(c: &mut Criterion) {
    let mut group = c.benchmark_group("divrem_bit");
    group.bench_function("u32", |b| b.iter(|| black_box(div10u32(123456789))));
    group.bench_function("u64", |b| b.iter(|| black_box(div10u64(123456789012345))));

    group.finish();
}

fn bench_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("u64_length");
    group.bench_function("10000", |b| b.iter(|| black_box(u64_length(10000))));
    group.bench_function("99999999", |b| b.iter(|| black_box(u64_length(99999999))));
    group.bench_function("100000000", |b| b.iter(|| black_box(u64_length(100000000))));
    group.bench_function("100000000000000", |b| b.iter(|| black_box(u64_length(100000000000000))));
    group.bench_function("100000000000000000", |b| b.iter(|| black_box(u64_length(100000000000000000))));
    group.finish();
}

criterion_group!(
    benches, 
    bench_divrem_bit,
    bench_length,
);
criterion_main!(benches);
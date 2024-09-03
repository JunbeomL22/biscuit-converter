use criterion::{criterion_group, criterion_main, Criterion, black_box};
use flashparser::FlashParser;

fn bench_int_conv_1234(c: &mut Criterion) {
    let flash = FlashParser::default();
    let mut group = c.benchmark_group("1234");
    group.bench_function("flashparser", |b| b.iter(|| black_box(flash.to_u16("1234"))));
    group.bench_function("std", |b| b.iter(|| black_box("1234".parse::<u16>().unwrap())));
    group.bench_function("atoi", |b| b.iter(|| black_box(atoi::atoi::<u16>("1234".as_bytes()).unwrap())));
    group.finish();
}

fn bench_int_conv_123456789(c: &mut Criterion) {
    let flash = FlashParser::default();
    let mut group = c.benchmark_group("123456789");
    group.bench_function("flashparser", |b| b.iter(|| black_box(flash.to_u32("123456789"))));
    group.bench_function("std", |b| b.iter(|| black_box("123456789".parse::<u32>().unwrap())));
    group.bench_function("atoi", |b| b.iter(|| black_box(atoi::atoi::<u32>("123456789".as_bytes()).unwrap())));
    group.finish();
}

fn bench_int_conv_123456789012345(c: &mut Criterion) {
    let flash = FlashParser::default();
    let mut group = c.benchmark_group("123456789012345");
    group.bench_function("flashparser", |b| b.iter(|| black_box(flash.to_u64("123456789012345"))));
    group.bench_function("std", |b| b.iter(|| black_box("123456789012345".parse::<u64>().unwrap())));
    group.bench_function("atoi", |b| b.iter(|| black_box(atoi::atoi::<u64>("123456789012345".as_bytes()).unwrap())));
    group.finish();
}

fn bench_int_conv_1234567890123456789012345(c: &mut Criterion) {
    let flash = FlashParser::default();
    let mut group = c.benchmark_group("1234567890123456789012345");
    group.bench_function("flashparser", |b| b.iter(|| black_box(flash.to_u128("1234567890123456789012345"))));
    group.bench_function("std", |b| b.iter(|| black_box("1234567890123456789012345".parse::<u128>().unwrap())));
    group.bench_function("atoi", |b| b.iter(|| black_box(atoi::atoi::<u128>("1234567890123456789012345".as_bytes()).unwrap())));
    group.finish();
}

criterion_group!(
    benches, 
    bench_int_conv_1234, 
    bench_int_conv_123456789, 
    bench_int_conv_123456789012345, 
    bench_int_conv_1234567890123456789012345,
);
criterion_main!(benches);
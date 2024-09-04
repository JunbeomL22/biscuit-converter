use criterion::{criterion_group, criterion_main, Criterion, black_box};
use flashparser::FlashParser;

fn bench_int_conv_1234(c: &mut Criterion) {
    let flash = FlashParser::default();
    let mut group = c.benchmark_group("-1234");
    group.bench_function("flashparser", |b| b.iter(|| black_box(flash.to_i16("-1234"))));
    group.bench_function("std", |b| b.iter(|| black_box("-1234".parse::<i16>().unwrap())));
    group.bench_function("atoi", |b| b.iter(|| black_box(atoi::atoi::<i16>("-1234".as_bytes()).unwrap())));
    group.finish();
}

fn bench_int_conv_123456789(c: &mut Criterion) {
    let flash = FlashParser::default();
    let mut group = c.benchmark_group("-123456789");
    group.bench_function("flashparser", |b| b.iter(|| black_box(flash.to_i32("-123456789"))));
    group.bench_function("std", |b| b.iter(|| black_box("-123456789".parse::<i32>().unwrap())));
    group.bench_function("atoi", |b| b.iter(|| black_box(atoi::atoi::<i32>("-123456789".as_bytes()).unwrap())));
    group.finish();
}

fn bench_int_conv_123456789012345(c: &mut Criterion) {
    let flash = FlashParser::default();
    let mut group = c.benchmark_group("-123456789012345");
    group.bench_function("flashparser", |b| b.iter(|| black_box(flash.to_i64("-123456789012345"))));
    group.bench_function("std", |b| b.iter(|| black_box("-123456789012345".parse::<i64>().unwrap())));
    group.bench_function("atoi", |b| b.iter(|| black_box(atoi::atoi::<i64>("-123456789012345".as_bytes()).unwrap())));
    group.finish();
}

fn bench_int_conv_1234567890123456789012345(c: &mut Criterion) {
    let flash = FlashParser::default();
    let mut group = c.benchmark_group("-12345678901234567890123456789012");
    group.bench_function("flashparser", |b| b.iter(|| black_box(flash.to_i128("-12345678901234567890123456789012"))));
    group.bench_function("std", |b| b.iter(|| black_box("-12345678901234567890123456789012".parse::<i128>().unwrap())));
    group.bench_function("atoi", |b| b.iter(|| black_box(atoi::atoi::<i128>("-12345678901234567890123456789012".as_bytes()).unwrap())));
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
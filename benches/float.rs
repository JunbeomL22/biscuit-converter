use criterion::{criterion_group, criterion_main, Criterion, black_box};
use flashparser::FlashParser;

fn bench_float(c: &mut Criterion) {
    let x = "12345678901234.012345678";
    let mut group = c.benchmark_group("12345678901234.012345678");
    group.bench_function("std", |b| {
        b.iter(|| 
            black_box(x.parse::<f64>().unwrap()))
    });

    group.bench_function("flashparser", |b| {
        let parser = FlashParser::default();
        b.iter(|| 
            black_box(parser.nonnegative_float_to_u128(x) as f64))
    });

    group.bench_function("with fraction given", |b| {
        let mut parser = FlashParser::default();
        parser.with_fraction_length(9);
        b.iter(|| 
            black_box(parser.nonnegative_float_to_u128(x) as f64))
    });

    group.finish();
}

criterion_group!(
    benches, 
    bench_float,
);

criterion_main!(benches);
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use ryu;
use dtoa;
use itoa;

fn bench_int(c: &mut Criterion) {
    let x: i32 = 1234;
    let mut group = c.benchmark_group("1234");
    group.bench_function("to_string", |b| { 
        b.iter(|| 
            black_box(x).to_string())
    });
        
    group.bench_function("itoa", |b| {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let _ = buffer.format(black_box(x));
        })
    });
    group.finish();
    
    let x: i32 = 123456789;
    let mut group = c.benchmark_group("123456789");
    group.bench_function("to_string", |b| { 
        b.iter(|| 
            black_box(x).to_string())
    });
        
    group.bench_function("itoa", |b| {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let _ = buffer.format(black_box(x));
        })
    });

    group.finish();
    
    let x: i64 = 1234567890123345;
    let mut group = c.benchmark_group("1234567890123345");
    group.bench_function("to_string", |b| {
        b.iter(|| 
            black_box(x).to_string())
    });

    group.bench_function("itoa", |b| {
        let mut buffer = itoa::Buffer::new();
        b.iter(|| {
            let _ = buffer.format(black_box(x));
        })
    });

    group.finish();
}

fn bench_float(c: &mut Criterion) {
    let x: f64 = 123456789.012345;
    let mut group = c.benchmark_group("123456789.012345");
    group.bench_function("to_string", |b| {
        b.iter(|| 
            black_box(x).to_string())
    });

    group.bench_function("ryu", |b| {
        let mut buffer = ryu::Buffer::new();
        b.iter(|| {
            let _ = buffer.format(black_box(x));
        })
    });

    group.bench_function("dtoa", |b| {
        let mut buffer = ryu::Buffer::new();
        b.iter(|| {
            let _ = buffer.format(black_box(x));
        })
    });

    group.finish();
}

criterion_group!(
    benches, 
    bench_float,
    bench_int,
);

criterion_main!(benches);


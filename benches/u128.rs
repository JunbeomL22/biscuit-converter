use criterion::{criterion_group, criterion_main, Criterion, black_box};
use biscuit_converter::BiscuitConverter;
use atoi::atoi;

fn bench_u128(c: &mut Criterion) {
    let biscuit_converter = BiscuitConverter::default();
    
    let test_set: Vec<&str> = vec![
        "1",
        "12",
        "123", 
        "1234",
        "12345",
        "123456",
        "1234567",
        "12345678",
        "123456789",
        "1234567890",
        "12345678901",
        "123456789012",
        "1234567890123",
        "12345678901234",
        "123456789012345",
        "1234567890123456",
        "12345678901234567",
        "123456789012345678",
        "1234567890123456789",
        "12345678901234567890",
        "123456789012345678901",
        "1234567890123456789012",
        "12345678901234567890123",
        "123456789012345678901234",
        "1234567890123456789012345",
        "12345678901234567890123456",
        "123456789012345678901234567",
        "1234567890123456789012345678",
        "12345678901234567890123456789",
        "123456789012345678901234567890",
        "1234567890123456789012345678901",
        "12345678901234567890123456789012",
        "123456789012345678901234567890123",
        "1234567890123456789012345678901234",
        "12345678901234567890123456789012345",
        "123456789012345678901234567890123456",
        "1234567890123456789012345678901234567",
        "12345678901234567890123456789012345678",
        "123456789012345678901234567890123456789",
    ];
    
    for test in test_set {
        let mut group = c.benchmark_group(format!("u128 {}", test));
        
        let input_bytes = test.as_bytes();
        group.bench_function("biscuit", |b| b.iter(|| biscuit_converter.to_u128_decimal(black_box(input_bytes)).unwrap()));
        group.bench_function("std", |b| b.iter(|| black_box(test).parse::<u128>().unwrap()));
        group.bench_function("atoi", |b| b.iter(|| atoi::<u128>(black_box(input_bytes)).unwrap()));

        group.finish();
    }
}

criterion_group!(
    benches, 
    bench_u128,
);

criterion_main!(benches);
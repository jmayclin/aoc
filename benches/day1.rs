use aoc2023::day1::{day1_p1, rust_day1_p1};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read("resources/d1_input.txt").unwrap();
    let ptr = input.as_ptr();
    let len = input.len() as u64;
    c.bench_function("assembly", |b| b.iter(|| unsafe { day1_p1(ptr, len) }));
    c.bench_function("rust", |b| b.iter(|| rust_day1_p1(ptr, len)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

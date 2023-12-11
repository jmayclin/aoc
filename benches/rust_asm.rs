use aoc2023::{day1::{day1_p1, rust_day1_p1}, day6::{day6_p2, rust_day6_p2}};
use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn day1_p1_benchmark(c: &mut Criterion) {
    let input = std::fs::read("resources/d1_input.txt").unwrap();
    let ptr = input.as_ptr();
    let len = input.len() as u64;
    c.bench_function("assembly", |b| b.iter(|| unsafe { day1_p1(ptr, len) }));
    c.bench_function("rust", |b| b.iter(|| rust_day1_p1(ptr, len)));
}

fn day6_p2_benchmark(c: &mut Criterion) {
    let input = std::fs::read("resources/d6_input.txt").unwrap();
    let ptr = input.as_ptr();
    let len = input.len() as u64;
    c.bench_function("assembly", |b| b.iter(|| unsafe { day6_p2(ptr, len) }));
    c.bench_function("rust", |b| b.iter(|| rust_day6_p2(ptr, len)));
}

criterion_group!(benches, day1_p1_benchmark, day6_p2_benchmark);
criterion_main!(benches);

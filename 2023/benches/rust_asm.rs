use aoc2023::{day1::{day1_p1, rust_day1_p1}, day6::{day6_p2, rust_day6_p2}, day8::{day8_p1, rust_day8_p1, rust_day8_p1_packed}};
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

fn day8_p1_benchmark(c: &mut Criterion) {
    let input = std::fs::read("resources/d8_input.txt").unwrap();
    let ptr = input.as_ptr();
    let len = input.len() as u64;
    c.bench_function("d8-assembly", |b| b.iter(|| unsafe { day8_p1(ptr, len) }));
    c.bench_function("d8-rust", |b| b.iter(|| rust_day8_p1(ptr, len)));
    c.bench_function("d8-alternate", |b| b.iter(|| rust_day8_p1_packed(ptr, len)));
}

criterion_group!(benches, day1_p1_benchmark, day6_p2_benchmark, day8_p1_benchmark);
criterion_main!(benches);

use aoc2025::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench]
fn d1_p1() -> i64 {
    day1::d1_p1(day1::P1_INPUT)
}

#[divan::bench]
fn d1_p1_asm() -> u64 {
    unsafe {day1::d1_p1_asm(day1::P1_INPUT.as_ptr(), day1::P1_INPUT.len() as u64)}
}

#[divan::bench]
fn d1_p2() -> i32 {
    day1::d1_p2(day1::P1_INPUT)
}

#[divan::bench]
fn d2_p1() -> u64 {
    day2::d2_p1(day2::P2_INPUT)
}

#[divan::bench]
fn d2_p2() -> u64 {
    day2::d2_p2(day2::P2_INPUT)
}


#[divan::bench]
fn d3_p1() -> u64 {
    day3::p1(day3::INPUT)
}

#[divan::bench]
fn d3_p2() -> u64 {
    day3::p2(day3::INPUT)
}


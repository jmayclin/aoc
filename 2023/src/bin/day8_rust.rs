use aoc2023::day8::{day8_p1, rust_day8_p1_packed};

use crabgrind as cg;

// valgrind --tool=callgrind  --dump-instr=yes ./target/release/day8_rust
// callgrind_annotate callgrind.out.1366273 > output.txt

fn main() {
    let input = include_bytes!("../../resources/d8_input.txt");
    //let input = std::fs::read("resources/d8_input.txt").unwrap();
    let ptr = input.as_ptr();
    let len = input.len() as u64;
    //cg::callgrind::zero_stats();
    let res = rust_day8_p1_packed(ptr, len);
    //cg::callgrind::dump_stats("rust-performance");
    // rust_day8_p1_packed
}

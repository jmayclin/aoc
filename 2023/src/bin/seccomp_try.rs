use aoc2023::day8::{day8_p1, rust_day8_p1};
use std::{fs, time::Instant};

fn main() {
    // Initialize Python interpreter
    let input_data = fs::read_to_string("resources/d8_input.txt").expect("Failed to read input file");
    //let code = fs::read_to_string("d8.py").unwrap();
    
    //prctl::set_seccomp_strict().unwrap();
    println!("I TURNED ON SECCOMP");
    let vec = vec![0, 1, 2, 3, 4];
    println!("I made a vec");
    //let result = rust_day8_p1(input_data.as_bytes().as_ptr(), input_data.len() as u64);
    let result: u64 = unsafe { day8_p1(input_data.as_bytes().as_ptr(), input_data.len() as u64) };
    println!("result was {}", result);
    return;
}

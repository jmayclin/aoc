use aoc2023::day8::day8_p1;

fn main() {
    let input = include_bytes!("../../resources/d8_input.txt");
    //let input = std::fs::read("resources/d8_input.txt").unwrap();
    let ptr = input.as_ptr();
    let len = input.len() as u64;
    let res = unsafe { day8_p1(ptr, len) };
    // rust_day8_p1_packed
}

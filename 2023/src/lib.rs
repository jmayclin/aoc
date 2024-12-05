pub mod day1;
pub mod day2;
pub mod day6;
pub mod day8;

fn input(name: &str) -> Vec<u8> {
    std::fs::read(name).unwrap()
}
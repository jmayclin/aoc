// global ASM is rude to the debug information that
// I would like to have, and doesn't seem to play nicely
// with GDB.
// global_asm!(include_str!("day1.s"));

use std::ops::Deref;

extern "C" {
    pub fn day1_p1(input: *const u8, len: u64) -> u64;
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;

    #[test]
    fn other_test() {
        let input = 
        "1   5\n2   4\n";
        let result = unsafe { day1_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 2);

    }

    #[test]
    fn p1_asm_sample() {
        let input = input("resources/d1_sample.txt");
        println!("input: {:?}", input);
        let result = unsafe { day1_p1(input.as_ptr(), input.len() as u64) };
        assert!(result != 6);
        assert_eq!(result, 11);
    }

    // #[test]
    // fn p1_rust_sample() {
    //     let input = input("resources/d1_sample.txt");
    //     let result =rust_day1_p1(input.as_ptr(), input.len() as u64);
    //     assert_eq!(result, 142);
    // }

    #[test]
    fn p1_asm() {
        let input = input("resources/d1_input.txt");
        let result = unsafe { day1_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 2_000);
    }
}

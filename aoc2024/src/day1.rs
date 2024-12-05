// global ASM is rude to the debug information that
// I would like to have, and doesn't seem to play nicely
// with GDB.
// global_asm!(include_str!("day1.s"));

extern "C" {
    pub fn day1_p1(input: *const u8, len: u64) -> u64;
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;

    #[test]
    fn p1_asm_sample() {
        let input = input("resources/d1_sample.txt");
        println!("input: {:?}", input);
        let result = unsafe { day1_p1(input.as_ptr(), input.len() as u64) };
        //assert_eq!(result, 9);
    }

    // #[test]
    // fn p1_rust_sample() {
    //     let input = input("resources/d1_sample.txt");
    //     let result =rust_day1_p1(input.as_ptr(), input.len() as u64);
    //     assert_eq!(result, 142);
    // }

    // #[test]
    // fn p1_asm() {
    //     let input = input("resources/d1_input.txt");
    //     let result = unsafe { day1_p1(input.as_ptr(), input.len() as u64) };
    //     //assert_eq!(result, 2_000);
    // }
}

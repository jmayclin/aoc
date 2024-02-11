extern "C" {
    pub fn day8_p1(input: *const u8, len: u64) -> u64;
    // pub fn day6_p2(input: *const u8, len: u64) -> u64;
}


#[cfg(test)]
mod test {
    use crate::input;
    use super::*;

    //#[test]
    fn p1_asm_sample() {
        let input = input("resources/d8_sample.txt");
        let result = unsafe { day8_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 288);
    }

    //#[test]
    fn p1_asm() {
        let input = input("resources/d8_input.txt");
        let result = unsafe { day8_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 2065338);
    }

    // #[test]
    // fn p2_asm_sample() {
    //     let input = input("resources/d6_sample.txt");
    //     let result = unsafe { day6_p2(input.as_ptr(), input.len() as u64) };
    //     let rust_result = rust_day6_p2(input.as_ptr(), input.len() as u64);
    //     assert_eq!(result, 71503);
    //     assert_eq!(rust_result, 71503);
    // }
//
    // #[test]
    // fn p2_asm() {
    //     let input = input("resources/d6_input.txt");
    //     let result = unsafe { day6_p2(input.as_ptr(), input.len() as u64) };
    //     let rust_result = rust_day6_p2(input.as_ptr(), input.len() as u64);
    //     assert_eq!(rust_result, result);
    //     assert_ne!(result, 34934175); // wrong answer, using singles
    //     assert_eq!(result, 34934171); // right answer, using double
    // }
}

extern "C" {
    pub fn day2_p1(input: *const u8, len: u64) -> u64;
    pub fn day2_p2(input: *const u8, len: u64) -> u64;
}

#[cfg(test)]
mod test {
    use crate::input;
    use super::*;

    #[test]
    fn p1_asm_sample() {
        let input = input("resources/d2_sample.txt");
        let result = unsafe { day2_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 8);
    }

    #[test]
    fn p1_asm() {
        let input = input("resources/d2_input.txt");
        let result = unsafe { day2_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 1867);
    }

    #[test]
    fn p2_asm_sample() {
        let input = input("resources/d2_sample.txt");
        let result = unsafe { day2_p2(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 2286);
    }

    #[test]
    fn p2_asm() {
        let input = input("resources/d2_input.txt");
        let result = unsafe { day2_p2(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 84538);
    }
}
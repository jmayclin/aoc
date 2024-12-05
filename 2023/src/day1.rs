// global ASM is rude to the debug information that
// I would like to have, and doesn't seem to play nicely
// with GDB.
// global_asm!(include_str!("day1.s"));

extern "C" {
    pub fn day1_p1(input: *const u8, len: u64) -> u64;
}

pub fn rust_day1_p1(input: *const u8, len: u64) -> u64 {
    let slice = unsafe {std::slice::from_raw_parts(input, len as usize)};
    let slice = unsafe {std::str::from_utf8_unchecked(slice)};
    slice.lines().map(|line| {
        let mut left = 0;
        let mut right = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if left == 0 {
                    left = c.to_digit(10).unwrap();
                }
                right = c.to_digit(10).unwrap();
            }
        }
        (left * 10) as u64 + right as u64
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;

    #[test]
    fn p1_asm_sample() {
        let input = input("resources/d1_sample.txt");
        let result = unsafe { day1_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 142);
    }

    #[test]
    fn p1_rust_sample() {
        let input = input("resources/d1_sample.txt");
        let result =rust_day1_p1(input.as_ptr(), input.len() as u64);
        assert_eq!(result, 142);
    }

    #[test]
    fn p1_rust() {
        let input = input("resources/d1_input.txt");
        let result =rust_day1_p1(input.as_ptr(), input.len() as u64);
        assert_eq!(result, 54597);
    }

    #[test]
    fn p1_asm() {
        let input = input("resources/d1_input.txt");
        let result = unsafe { day1_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 54597);
    }
}

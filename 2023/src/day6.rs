extern "C" {
    pub fn day6_p1(input: *const u8, len: u64) -> u64;
    pub fn day6_p2(input: *const u8, len: u64) -> u64;
}

pub fn rust_day6_p2(input: *const u8, len: u64) -> u64 {
    let slice = unsafe {std::slice::from_raw_parts(input, len as usize)};
    let slice = unsafe {std::str::from_utf8_unchecked(slice)};
    let mut lines = slice.lines();
    let time: String = lines.next().unwrap().chars().filter(|c| c.is_ascii_digit()).collect();
    let time = u64::from_str_radix(&time, 10).unwrap();

    let distance: String = lines.next().unwrap().chars().filter(|c| c.is_ascii_digit()).collect();
    let distance = u64::from_str_radix(&distance, 10).unwrap();
    num_solutions(time, distance)

}

fn num_solutions(time: u64, distance: u64) -> u64 {
    let time_f = time as f64;
    let distance_f = distance as f64;
    let term1 = time_f / 2.0;
    let term2 = time_f.powi(2) - 4.0 * distance_f;
    let term2 = term2.sqrt() / 2.0;
    let higher = term1 + term2;
    let lower = term1 - term2;
    let higher = higher.floor() as u64;
    let lower = lower.ceil() as u64;
    let mut solutions = higher - lower + 1;
    if (time - lower) * lower == distance {
        solutions -= 2;
    }
    solutions
}

#[cfg(test)]
mod test {
    use crate::input;
    use super::*;

    #[test]
    fn p1_asm_sample() {
        let input = input("resources/d6_sample.txt");
        let result = unsafe { day6_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 288);
    }

    #[test]
    fn p1_asm() {
        let input = input("resources/d6_input.txt");
        let result = unsafe { day6_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 2065338);
    }

    #[test]
    fn p2_asm_sample() {
        let input = input("resources/d6_sample.txt");
        let result = unsafe { day6_p2(input.as_ptr(), input.len() as u64) };
        let rust_result = rust_day6_p2(input.as_ptr(), input.len() as u64);
        assert_eq!(result, 71503);
        assert_eq!(rust_result, 71503);
    }

    #[test]
    fn p2_asm() {
        let input = input("resources/d6_input.txt");
        let result = unsafe { day6_p2(input.as_ptr(), input.len() as u64) };
        let rust_result = rust_day6_p2(input.as_ptr(), input.len() as u64);
        assert_eq!(rust_result, result);
        assert_ne!(result, 34934175); // wrong answer, using singles
        assert_eq!(result, 34934171); // right answer, using double
    }
}
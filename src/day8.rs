use std::collections::HashMap;

extern "C" {
    pub fn day8_p1(input: *const u8, len: u64) -> u64;
    // pub fn day6_p2(input: *const u8, len: u64) -> u64;
}

fn index(letters: &str) -> u64 {
    let bytes = letters.as_bytes();
    (bytes[0] - b'A') as u64 * 26 * 26 + (bytes[1] - b'A') as u64 * 26 + (bytes[2] - b'A') as u64
}


pub fn rust_day8_p1(input: *const u8, len: u64) -> u64 {
    let slice = unsafe {std::slice::from_raw_parts(input, len as usize)};
    let slice = unsafe {std::str::from_utf8_unchecked(slice)};
    let mut lines = slice.lines();
    let input = lines.next().unwrap().bytes().cycle();
    lines.next(); // skip over the empty line
    let graph: HashMap<String, (String, String)> = lines.map(|l| {
        let key   = &l[0..3];
        let left  = &l[7..10];
        let right = &l[12..15];
        (key.to_owned(), (left.to_owned(), right.to_owned()))
    }).collect();
    
    let mut current = "AAA";
    let mut count = 0;
    for step in input {
        if current == "ZZZ" {
            return count;
        }
        if step == b'L' {
            current = &graph.get(current).unwrap().0;
        } else {
            current = &graph.get(current).unwrap().1;
        }
        count += 1;
    }
    return 0;
}


#[cfg(test)]
mod test {
    use std::time::Instant;

    use crate::input;
    use super::*;

    #[test]
    fn p1_asm_sample() {
        let input = input("resources/d8_sample.txt");
        let result = unsafe { day8_p1(input.as_ptr(), input.len() as u64) };
        assert_eq!(result, 6);
        assert_eq!(result, rust_day8_p1(input.as_ptr(), input.len() as u64));
    }

    #[test]
    fn p1_asm() {
        let input = input("resources/d8_input.txt");
        let assembly = Instant::now();

        let result = unsafe { day8_p1(input.as_ptr(), input.len() as u64) };
        let assembly_done = assembly.elapsed();
        assert_eq!(result, 13019);
        let rust = Instant::now();
        let rust_result = rust_day8_p1(input.as_ptr(), input.len() as u64);
        let rust_done = rust.elapsed();
        print!("assembly:{}, rust:{}", assembly_done.as_micros(), rust_done.as_micros());
        assert!(false);

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

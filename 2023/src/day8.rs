use std::collections::{BTreeMap, HashMap};

extern "C" {
    pub fn day8_p1(input: *const u8, len: u64) -> u64;
    // pub fn day6_p2(input: *const u8, len: u64) -> u64;
}

fn index(letters: &str) -> usize {
    let bytes = letters.as_bytes();
    (bytes[0] - b'A') as usize * 26 * 26 + (bytes[1] - b'A') as usize * 26 + (bytes[2] - b'A') as usize
    // ((bytes[0] - b'A') * 26 * 26 + (bytes[1] - b'A') * 26 + (bytes[2] - b'A')) as usize

}

// initial -> 113.93 us
// remove initial loop -> 84.7 us
// manual loop unroll -> 120 us
pub fn rust_day8_p1_packed(input: *const u8, len: u64) -> u64 {
    let slice = unsafe {std::slice::from_raw_parts(input, len as usize)};
    let slice = unsafe {std::str::from_utf8_unchecked(slice)};
    let mut lines = slice.lines();
    let input = lines.next().unwrap().bytes().cycle();
    lines.next(); // skip over the empty line
    
    let mut key_locations = HashMap::new();
    let mut graph: Vec<(usize, usize)> = lines.map(|l| {
        let key   = index(&l[0..3]);
        let left  = index(&l[7..10]);
        let right = index(&l[12..15]);
        (key, (left, right))
    })
    .enumerate()
    .map(|(i, (key, (left, right)))| {
        key_locations.insert(key, i);
        (left, right)
    })
    .collect();

    for i in 0..graph.len() {
        graph[i].0 = *key_locations.get(&graph[i].0).unwrap();
        graph[i].1 = *key_locations.get(&graph[i].1).unwrap();
    }

    let mut current = *key_locations.get(&index("AAA")).unwrap();
    let end = *key_locations.get(&index("ZZZ")).unwrap();
    let mut count = 0;
    for step in input {
        if current == end {
            return count;
        }
        if step == b'L' {
            current = graph[current].0;
        } else {
            current = graph[current].1;
        }
        count += 1;
    }
    return 0;
}

// btree &str -> 900 us
// hashmap &str -> 406.26 us
// hashmap usize -> 298.61 us
pub fn rust_day8_p1(input: *const u8, len: u64) -> u64 {
    let slice = unsafe {std::slice::from_raw_parts(input, len as usize)};
    let slice = unsafe {std::str::from_utf8_unchecked(slice)};
    let mut lines = slice.lines();
    let input = lines.next().unwrap().bytes().cycle();
    lines.next(); // skip over the empty line
    let graph: HashMap<usize, (usize, usize)> = lines.map(|l| {
        let key   = index(&l[0..3]);
        let left  = index(&l[7..10]);
        let right = index(&l[12..15]);
        //(key.to_owned(), (left.to_owned(), right.to_owned()))
        (key, (left, right))
    }).collect();
    
    let mut current = index("AAA");
    let mut count = 0;
    for step in input {
        if current == index("ZZZ") {
            return count;
        }
        if step == b'L' {
            current = graph.get(&current).unwrap().0;
        } else {
            current = graph.get(&current).unwrap().1;
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
        println!("assembly:{:?}, rust:{:?}", assembly_done, rust_done);
        //assert!(false);

    }

    #[test]
    fn packed_rust() {
        let input = input("resources/d8_input.txt");
        let result = rust_day8_p1_packed(input.as_ptr(), input.len() as u64);
        assert_eq!(result, 13019);
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

use std::{arch::global_asm, io::BufRead};

const DIAL_START: i64 = 50;

global_asm!(
    r#"
    .global d1_p1_asm
    d1_p1_asm:
        add x1, x0, x1                // x1 now stores "end" memory address
        mov x5, #10                   // CONSTANT: 10 for decimal number parsing
        mov x6, #-1                   // CONSTANT: -1 for sign flipping
        mov x7, #100                // CONSTANT: 100 for modulus flipping
        mov x8, #50                   // current value of dial
        mov x9, #0                    // accumulate total zeros
    parse_number:
        cmp x0, x1                    // check if we've reached the end
        b.eq finish

        ldrb w2, [x0], #1             // load the current byte into w2, then += 1 to counter
        ldrb w3, [x0], #1             // first byte of the number
        sub x3, x3, #48
    eat_digits:
        ldrb w4, [x0], #1
        cmp w4, #10                   // compare to \n
        b.eq adjust_sign
        sub x4, x4, #48
        mul x3, x3, x5                // multiply by 10
        add x3, x3, x4                // add previous number
        b eat_digits
    adjust_sign:
        cmp x2, #76                       // multiply by -1 if 'L'
        b.ne tally_clicks
        mul x3, x3, x6
    tally_clicks:
        add x8, x8, x3                // update click
        sdiv x10, x8, x7
        msub x8, x10, x7, x8
        cmp x8, #0
        b.ne parse_number
        add x9, x9, #1
        b parse_number
    finish:
        mov x0, x9
        ret
    "#,
);

unsafe extern "C" {
    fn d1_p1_asm(input: *const u8, length: u64) -> u64;
}

fn d1_p1(input: &[u8]) -> i64 {
    let mut i = 0_usize;
    let mut current = 50;
    let mut zeros_seen = 0;


    while i < input.len() {
        // parse sign
        let sign = if input[i] == b'L' { 1 } else { -1 };
        i += 1;
        
        let number: i32 = {
            // parse number
            let mut number = 0;
            loop {
                number += (input[i] - b'0') as i32;
                i += 1;
                if input[i] == b'\n' {
                    i += 1;
                    break;
                }
                number *= 10;
            }
            number * sign
        };

        // at exit of loop, i is pointer to next R/L
        current += number;
        current = current.rem_euclid(100);
        if current == 0 {
            zeros_seen += 1;
        }

    }
    zeros_seen
}

fn assert_less_than_u16(input: &[u8]) {
    let str = String::from_utf8(input.to_vec()).unwrap();
    let turns: Vec<i64> = str
        .lines()
        .map(|line| {
            let direction = line.as_bytes()[0];
            let remaining = &line[1..];
            let mut clicks: i64 = remaining.parse().unwrap();
            if direction == b'R' {
                clicks *= -1;
            }
            clicks
        })
        .collect();

    for click in turns {
        assert!((click.abs() as u16) < u16::MAX);
    }
}

fn d1_p2(input: &[u8]) -> u32 {
    // assumption: numbers all fit in u16
    if cfg!(debug_assertions) {
        assert_less_than_u16(input);
    }

    let str = String::from_utf8(input.to_vec()).unwrap();

    let mut sign: Vec<i8> = Vec::new();
    let mut magnitude: Vec<u16> = Vec::new();

    for line in str.lines() {
        sign.push(if line.as_bytes()[0] == b'R' { 1 } else { -1 });
        magnitude.push((&line[1..]).parse().unwrap());
    }

    let easy_turns: u16 = magnitude.iter().map(|turns| turns / 100).sum();
    let clicks: Vec<i32> = magnitude
        .into_iter()
        .map(|m| m % 100)
        .zip(sign.into_iter())
        .map(|(magnitude, direction)| magnitude as i32 * direction as i32)
        .collect();

    let mut zeros_seen: u32 = 0;

    let mut current: i32 = 50;
    for click in clicks {
        // -99 to 99
        let was_zero = current == 0;

        current += click;
        let pre_modulo = current;
        current = current.rem_euclid(100);

        // 90 + 5 -> 95, no zero pass
        // 90 + 11 -> 101, a zero pass
        if pre_modulo != current && current != 0 && !was_zero {
            zeros_seen += 1;
        }
        if current == 0 {
            zeros_seen += 1;
        }
    }
    zeros_seen + easy_turns as u32
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    const P1_INPUT_SAMPLE: &[u8] = include_bytes!("../resources/d1_p1_sample.txt");
    const P1_INPUT: &[u8] = include_bytes!("../resources/d1_p1.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(d1_p1(P1_INPUT_SAMPLE), 3)
    }

    // 120 us -> 130us
    // no change with rem_euclidian behavior?
    #[test]
    fn p1() {
        let start = Instant::now();
        let result = d1_p1(P1_INPUT);
        let elapsed = start.elapsed();
        println!("p1 took {:?}", elapsed);
        assert_eq!(result, 1097)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(d1_p2(P1_INPUT_SAMPLE), 6)
    }

    // 120 us -> 130us
    // no change with rem_euclidian behavior?
    #[test]
    fn p2() {
        let start = Instant::now();
        let result = d1_p2(P1_INPUT);
        let elapsed = start.elapsed();
        println!("took {:?}", elapsed);
        assert_eq!(result, 7101)
    }

    #[test]
    fn mod_understanding() {
        assert_eq!(-10 % 100, -10);
        assert_eq!(-100 % 100, 0);
        assert_eq!(-110 % 100, -10);
    }

    /// gdb --args target/debug/deps/aoc2023-b86eedb9bba114e8 day1
    #[test]
    fn p1_asm_sample() {
        let slice = P1_INPUT_SAMPLE;
        let result = unsafe {d1_p1_asm(slice.as_ptr(), slice.len() as u64)};
        assert_eq!(result, 3)
    }

    #[test]
    fn p1_asm() {
        let slice = P1_INPUT;
        let start = Instant::now();
        let result = unsafe {d1_p1_asm(slice.as_ptr(), slice.len() as u64)};
        let elapsed = start.elapsed();
        println!("p1 asm took {elapsed:?}");
        assert_eq!(result, 1097)
    }
}

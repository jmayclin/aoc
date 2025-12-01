use std::io::BufRead;

const DIAL_START: i64 = 50;

fn d1_p1(input: &[u8]) -> i64 {
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

    let mut zeros_seen = 0;
    let mut current = 50;
    for click in turns {
        current += click;
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
        sign.push(if line.as_bytes()[0] == b'R' {1} else {-1});
        magnitude.push((&line[1..]).parse().unwrap());
    }

    let easy_turns: u16 = magnitude.iter().map(|turns| turns / 100).sum();
    let clicks: Vec<i32> = magnitude
        .into_iter()
        .map(|m| m % 100)
        .zip(sign.into_iter())
        .map(|(magnitude, direction)| magnitude as i32 * direction as i32)
        .collect();

    // let turns: Vec<i64> = str
    //     .lines()
    //     .map(|line| {
    //         let direction = line.as_bytes()[0];
    //         let remaining = &line[1..];
    //         let mut clicks: i64 = remaining.parse().unwrap();
    //         if direction == b'R' {
    //             clicks *= -1;
    //         }
    //         clicks
    //     })
    //     .collect();

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
        println!("took {:?}", elapsed);
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
}

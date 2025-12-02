use std::{cmp::min_by, u64};

pub const P2_INPUT_SAMPLE: &[u8] = include_bytes!("../resources/d2_sample.txt");
pub const P2_INPUT: &[u8] = include_bytes!("../resources/d2_input.txt");

pub fn is_sequence_num(base: u64) -> bool {
    let base = base.to_string();
    if base.len() % 2 != 0 {
        return false;
    }

    let (upper, lower) = base.split_at(base.len() / 2);
    upper == lower
}
/// 10 -> 11
/// 123 -> 1000 -> 1010
///
/// 98 -> 99
///
/// prefer to "paste" lower
///
/// 101100 -> 101101
/// 102100 -> 101101
///
/// 1010 -> 1111
/// 1111 -> 1212
/// 1212 -> 1313
///
pub fn next_sequence_num(base: u64) -> u64 {
    let mut current = base;
    let mut digits = count_digits(current);
    if digits % 2 == 1 {
        // increment to the next power of 10
        current = exp_10(digits);
        digits += 1;
    }
    let mut upper_half = current / exp_10(digits / 2);
    // can we just "paste" to the lower side. This is the case if the
    // initial argument wasn't a sequence num
    let potential_sequence = upper_half * exp_10(digits / 2) + upper_half;
    if potential_sequence > base {
        potential_sequence
    } else {
        upper_half += 1;
        // if incrementing brings us to a power of 10, then we need to
        // double the digits
        if power_of_10(upper_half) {
            current = exp_10(digits + 1);
            digits += 2;
            let upper_half = current / exp_10(digits / 2);
            upper_half * exp_10(digits / 2) + upper_half
        } else {
            upper_half * exp_10(digits / 2) + upper_half
        }
    }
}

/// return the power of 10
///
/// This can be used to determine if a number is even or odd
pub fn count_digits(mut num: u64) -> u64 {
    let mut power = 0;
    while num >= 10 {
        num /= 10;
        power += 1;
    }
    power + 1
}

pub fn exp_10(power: u64) -> u64 {
    let mut result = 1;
    for i in 0..power {
        result *= 10;
    }
    result
}

pub fn power_of_10(mut value: u64) -> bool {
    if value % 10 != 0 {
        return false;
    }
    value /= 10;
    while value > 1 {
        if value % 10 != 0 {
            return false;
        }
        value /= 10;
    }
    return true;
}

/// return the next sequence using a sequence length of seq_len
///
/// 74016 -> 77777
/// 610629473, 1 -> 666666666
/// 610629473, 3 -> 610610610
/// 20011000, 1 -> 22222222
/// 20011000, 2 -> 20202020
/// 20011000, 4 -> 20011002
/// 999 -> 1000, 2 -> 1010  
///
/// If we hit a power of two bump, then I think it's a standard format
fn next_specific_sequence_num(value: u64, seq_len: u64) -> u64 {
    let digits = count_digits(value);
    if digits == 1 {
        return u64::MAX;
    }
    debug_assert!(seq_len <= digits / 2, "v:{value}, s:{seq_len}, {digits}");
    let upper_sequence = value / exp_10(digits - seq_len);
    // println!("v: {value}, sl: {seq_len}, digits: {digits}, us: {upper_sequence}");
    let copy_paste = {
        let mut start = upper_sequence;
        for i in 0..(digits / seq_len - 1) {
            start = start * exp_10(seq_len) + upper_sequence;
        }
        start
    };

    if copy_paste > value {
        // println!("returning copy-paste: {copy_paste}");
        return copy_paste;
    }

    let incremented = {
        let mut start = upper_sequence + 1;
        if power_of_10(start) {
            return u64::MAX;
        }
        for i in 0..(digits / seq_len - 1) {
            start = start * exp_10(seq_len) + (upper_sequence + 1)
        }
        start
    };
    // println!("returning incremented: {incremented}");
    incremented
}

/// 1000 -> 1010
/// 100  -> 111
fn first_power_of_10_sequence(digits: u64) -> u64 {
    let base = exp_10(digits - 1);
    if digits % 2 == 0 {
        next_specific_sequence_num(base, digits / 2)
    } else {
        next_specific_sequence_num(base, 1)
    }
}

fn next_any_length_sequence_num(value: u64) -> u64 {
    // possible lengths
    let current_digits = count_digits(value);
    //println!("checking {current_digits} for {value}");
    let mut best_option = first_power_of_10_sequence(current_digits + 1);
    // println!("best option {best_option}");
    let option = next_specific_sequence_num(value, 1);
    best_option = best_option.min(option);

    for i in 2..(count_digits(value) / 2 + 1) {
        let is_factor = {
            let n = current_digits / i;
            n * i == current_digits
        };
        if is_factor {
            let option = next_specific_sequence_num(value, i);
            best_option = best_option.min(option);
        }
    }
    debug_assert!(best_option > value);
    best_option
}

// pub fn fast_log(num: u64) -> u64 {
//     if num < 10 {
//         1
//     }
// }

pub fn d2_p1(input: &[u8]) -> u64 {
    (input)
        .split(|b| *b == b',')
        .map(|range| {
            let dash = range.iter().position(|b| *b == b'-').unwrap();
            let (start, end) = range.split_at(dash);
            let start: u64 = str::from_utf8(start).unwrap().parse().unwrap();
            let end: u64 = str::from_utf8(&end[1..]).unwrap().parse().unwrap();
            (start, end)
        })
        .map(|(start, end)| {
            let mut total = 0;
            let mut next_sequence = next_sequence_num(start - 1);
            while next_sequence <= end {
                total += next_sequence;
                next_sequence = next_sequence_num(next_sequence);
            }
            total as u64
        })
        .sum()
}

pub fn d2_p2(input: &[u8]) -> u64 {
    (input)
        .split(|b| *b == b',')
        .map(|range| {
            let dash = range.iter().position(|b| *b == b'-').unwrap();
            let (start, end) = range.split_at(dash);
            let start: u64 = str::from_utf8(start).unwrap().parse().unwrap();
            let end: u64 = str::from_utf8(&end[1..]).unwrap().parse().unwrap();
            (start, end)
        })
        .map(|(start, end)| {
            let mut total = 0;
            let mut next_sequence = next_any_length_sequence_num(start - 1);
            while next_sequence <= end {
                total += next_sequence;
                next_sequence = next_any_length_sequence_num(next_sequence);
            }
            total as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day2::next_sequence_num;

    use super::*;

    #[test]
    fn log() {
        assert_eq!(count_digits(1234), 4);
        assert_eq!(count_digits(1000), 4);
        assert_eq!(count_digits(9999), 4);
        assert_eq!(count_digits(101010), 6);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(6), 1);
        assert_eq!(count_digits(19), 2);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(193), 3);
        assert_eq!(count_digits(123), 3);
    }

    #[test]
    fn exp() {
        assert_eq!(exp_10(1), 10);
        assert_eq!(exp_10(2), 100);
        assert_eq!(exp_10(3), 1000);
    }

    #[test]
    fn power() {
        assert!(power_of_10(10));
        assert!(power_of_10(100));
        assert!(power_of_10(1000));
        assert!(!power_of_10(5));
        assert!(!power_of_10(50));
        assert!(!power_of_10(54));
    }

    #[test]
    fn next_sequence() {
        // 123 -> 1000 -> 1010
        // 1010 -> 1111
        // 1111 -> 1212
        // 1212 -> 1313
        assert_eq!(next_sequence_num(10), 11);
        assert_eq!(next_sequence_num(94), 99);
        assert_eq!(next_sequence_num(99), 1010);
        assert_eq!(next_sequence_num(123), 1010);
        assert_eq!(next_sequence_num(1010), 1111);
        assert_eq!(next_sequence_num(1111), 1212);
        assert_eq!(next_sequence_num(1212), 1313);

        assert_eq!(next_sequence_num(926387), 926926);
        assert_eq!(next_sequence_num(789123), 789789);
        assert_eq!(next_sequence_num(101002), 101101);
    }

    #[test]
    fn p1_sample() {
        assert_eq!(d2_p1(P2_INPUT_SAMPLE), 1227775554)
    }

    #[test]
    fn p1_input() {
        let result = d2_p1(P2_INPUT);
        assert_eq!(result, 20223751480)
    }

    #[test]
    fn exhaustive() {
        let mut next_sequence = 11;

        for i in 0..1_000_000 {
            if i == next_sequence {
                next_sequence = next_sequence_num(i);
                dbg!(next_sequence);
                assert!(is_sequence_num(next_sequence));
            } else {
                assert_eq!(next_sequence, next_sequence_num(i));
                assert!(!is_sequence_num(i))
            }
        }
    }

    /// return the next sequence using a sequence length of seq_len
    ///
    /// 74016 -> 77777
    /// 610629473, 1 -> 666666666
    /// 610629473, 3 -> 610610610
    /// 20011000, 1 -> 22222222
    /// 20011000, 2 -> 20202020
    /// 20011000, 4 -> 20011002
    /// 999 -> 1000, 2 -> 1010  
    ///
    /// If we hit a power of two bump, then I think it's a standard format
    #[test]
    fn specific_sequence() {
        assert_eq!(next_specific_sequence_num(74016, 1), 77777);
        assert_eq!(next_specific_sequence_num(610629473, 1), 666666666);
        assert_eq!(next_specific_sequence_num(610629473, 3), 611611611);
        assert_eq!(next_specific_sequence_num(20011000, 1), 22222222);
        assert_eq!(next_specific_sequence_num(20011000, 2), 20202020);
        assert_eq!(next_specific_sequence_num(20011000, 4), 20012001);
        
    }

    #[test]
    fn next_any_length() {
        assert_eq!(next_any_length_sequence_num(99), 111);
        assert_eq!(next_any_length_sequence_num(999), 1010);
        assert_eq!(next_any_length_sequence_num(824824821), 824824824);
        assert_eq!(next_any_length_sequence_num(824824824), 825825825);
        assert_eq!(next_any_length_sequence_num(1188511880), 1188511885);

        // 5542145-5582046,
        assert_eq!(next_any_length_sequence_num(5542145 - 1), 5555555);
        assert_eq!(next_any_length_sequence_num(5555555), 6666666);

        // 243-401,
        assert_eq!(next_any_length_sequence_num(243 - 1), 333);
        assert_eq!(next_any_length_sequence_num(333), 444);

        // 884211-917063,
        assert_eq!(next_any_length_sequence_num(884211 - 1), 884884);
        assert_eq!(next_any_length_sequence_num(884884), 885885);
        assert_eq!(next_any_length_sequence_num(885885), 886886);
        assert_eq!(next_any_length_sequence_num(889889), 890890);

        // 1174-1665,
        // 767028-791710,
        // 308275-370459,
        // 285243789-285316649,
        // 3303028-3361832,
        // 793080-871112,
        // 82187-123398
        assert_eq!(next_any_length_sequence_num(82187 - 1), 88888);
        assert_eq!(next_any_length_sequence_num(88888), 99999);
        assert_eq!(next_any_length_sequence_num(99999), 100100);
        assert_eq!(next_any_length_sequence_num(9999), 11111);
        assert_eq!(next_any_length_sequence_num(9999), 11111);
        assert_eq!(next_any_length_sequence_num(1), 11);
    }

    #[test]
    fn p2_sample() {
        assert_eq!(d2_p2(P2_INPUT_SAMPLE), 4174379265)
    }

    #[test]
    fn p2_input() {
        let result = d2_p2(P2_INPUT);
        assert_eq!(result, 30260171216)
    }
}

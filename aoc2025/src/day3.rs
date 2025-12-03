use std::{cmp::min_by, u64};

pub const SAMPLE: &[u8] = include_bytes!("../resources/d3_sample.txt");
pub const INPUT: &[u8] = include_bytes!("../resources/d3_input.txt");

/// Calculate the joltage.
///
/// 234234234234278
///
/// joltage(bank[])
fn joltage(bank: &[u8]) -> u8 {
    let last_two = [bank[bank.len() - 2], bank[bank.len() - 1]];

    let mut best_joltage: u8 = str::from_utf8(&last_two).unwrap().parse().unwrap();
    let mut best_seen_battery = last_two.iter().max().unwrap() - b'0';
    for battery in bank.iter().rev().skip(2).cloned() {
        let this_battery = battery - b'0';
        let this_joltage = this_battery * 10 + best_seen_battery;

        best_seen_battery = best_seen_battery.max(this_battery);
        best_joltage = best_joltage.max(this_joltage);
    }
    best_joltage
}

const JOLTAGE_SIZE: usize = 12;

/// 4 4511 1111 -> false
/// 4 4311 1111 -> true
///
/// 8 23411111 -> 83411111
/// 8 43211111 -> 84321111
fn can_improve_joltage(new_battery: u8, joltage: &[u8; JOLTAGE_SIZE]) -> bool {
    for battery in joltage {
        if new_battery > *battery {
            return true;
        }
        if new_battery < *battery {
            return false;
        }
    }
    return false;
}

/// Calculate the joltage.
///
/// 234234234234278
///
/// 4 4511 1111
/// 4 4311 1111
///
/// 8 23411111 -> 83411111
/// 8 43511111 -> 84511111
/// 8 43211111 -> 84321111
/// How do we choose which battery to turn off?
/// We will turn off the first battery which is less than it's peer, otherwise the
/// last one
///
/// so _if_ the current battery is greater than the current most significant digit
/// then we should will be able to improve the current value.
///
/// start with the base joltage.
/// For each subsequent battery we can either
///     A: turn on this battery any turn off _one_ other battery -> I think this might
///        cascade?
///     B: keep this battery turned off an use the previous
fn mega_joltage(bank: &[u8]) -> u64 {
    let mut joltage = [0; JOLTAGE_SIZE];
    joltage.copy_from_slice(&bank[bank.len() - JOLTAGE_SIZE..]);
    for j in joltage.iter_mut() {
        *j -= b'0';
    }

    for battery in bank.iter().rev().skip(12).cloned() {
        let this_battery = battery - b'0';
        // we can built a better joltage
        if can_improve_joltage(this_battery, &joltage) {
            let mut turn_off_index = JOLTAGE_SIZE - 1;

            let mut previous = joltage[0];
            for (i, joltage_battery) in joltage.iter().cloned().enumerate().skip(1) {
                if previous < joltage_battery {
                    turn_off_index = i - 1;
                    break;
                }
                previous = joltage_battery
            }

            // shift the batteries over
            if turn_off_index != 0 {
                joltage.copy_within(0..turn_off_index, 1);
            }
            joltage[0] = this_battery;
        }
    }
    let mut total = 0;
    for battery in joltage {
        total = total * 10 + battery as u64
    }
    total
}

fn view_joltage(joltage: &[u8; JOLTAGE_SIZE]) -> String {
    let digits: Vec<String> = joltage.iter().map(|b| (*b).to_string()).collect();
    digits.join("")
}

fn brute_force_joltage(bank: &[u8]) -> u64 {
    assert_eq!(bank.len(), JOLTAGE_SIZE + 3);
    let mut best_joltage = 0;
    for i in 0..JOLTAGE_SIZE {
        for j in (i + 1)..JOLTAGE_SIZE {
            for k in (j + 1)..JOLTAGE_SIZE {
                let joltage: Vec<u8> = bank
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter(|(index, value)| *index != i && *index != j && *index != k)
                    .map(|(index, battery)| battery)
                    .collect();
                let current_joltage = array_to_u64(&joltage);
                best_joltage = best_joltage.max(current_joltage);
            }
        }
    }
    best_joltage
}

fn array_to_u64(joltage: &[u8]) -> u64 {
    let mut total = 0;
    for battery in joltage {
        total = total * 10 + (*battery - b'0') as u64
    }
    total
}

pub fn p1(input: &[u8]) -> u64 {
    input
        .split(|b| *b == b'\n')
        .filter(|b| !b.is_empty())
        .map(|bank| joltage(bank) as u64)
        .sum()
}

pub fn p2(input: &[u8]) -> u64 {
    input
        .split(|b| *b == b'\n')
        .filter(|b| !b.is_empty())
        .map(|bank| mega_joltage(bank))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joltage_cases() {
        assert_eq!(joltage(b"987654321111111"), 98);
        assert_eq!(joltage(b"811111111111119"), 89);
        assert_eq!(joltage(b"234234234234278"), 78);
        assert_eq!(joltage(b"818181911112111"), 92);
    }

    #[test]
    fn mega_joltage_cases() {
        assert_eq!(mega_joltage(b"987654321111111"), 987654321111);
        assert_eq!(mega_joltage(b"811111111111119"), 811111111119);
        assert_eq!(mega_joltage(b"234234234234278"), 434234234278);
        assert_eq!(mega_joltage(b"818181911112111"), 888911112111);
    }

    #[test]
    fn diff_joltage() {
        let cases = [
            //b"987654321111111",
            //b"811111111111119",
            //b"234234234234278",
            //b"818181911112111",
            b"916592934156427",
            b"917472612392848",
        ];

        for case in cases {
            assert_eq!(mega_joltage(case), brute_force_joltage(case));
        }
    }

    #[test]
    fn p1_sample() {
        assert_eq!(p1(SAMPLE), 357)
    }

    #[test]
    fn p1_input() {
        let result = p1(INPUT);
        assert_eq!(result, 17493)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2(SAMPLE), 3121910778619)
    }

    #[test]
    fn p2_input() {
        assert_ne!(p2(INPUT), 173679513894872);
        assert_eq!(p2(INPUT), 173685428989126)
    }
}

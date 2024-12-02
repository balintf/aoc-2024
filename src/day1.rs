use std::collections::HashMap;
use memchr::memchr;
use rustc_hash::FxBuildHasher;

#[inline]
fn parse_digits(input: &[u8]) -> u64 {
    let mut result = 0;
    for byte in input {
        result = result * 10 + (byte - b'0') as u64;
    }
    result
}

pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut left = Vec::<u64>::with_capacity(1000);
    let mut right = Vec::<u64>::with_capacity(1000);
    let mut idx = 0;
    while idx < input.len() {
        let space_idx = unsafe { memchr(b' ', &input[idx..]).unwrap_unchecked() };
        let l = parse_digits(&input[idx..idx + space_idx]);
        left.push(l);
        idx = idx + space_idx + 1;
        while unsafe { *input.get_unchecked(idx) } == b' ' {
            idx += 1;
        }
        let newline_idx = unsafe { memchr(b'\n', &input[idx..]).unwrap_unchecked() };
        let r =parse_digits(&input[idx..idx + newline_idx]);
        right.push(r);
        idx = idx + newline_idx + 1;        
    }

    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut left = Vec::<u64>::with_capacity(1000);
    let mut right = HashMap::<u64, u32, FxBuildHasher>::with_capacity_and_hasher(1000, FxBuildHasher::default());
    let mut idx = 0;
    while idx < input.len() {
        let space_idx = unsafe { memchr(b' ', &input[idx..]).unwrap_unchecked() };
        let l = parse_digits(&input[idx..idx + space_idx]);
        left.push(l);
        idx = idx + space_idx + 1;
        while unsafe { *input.get_unchecked(idx) } == b' ' {
            idx += 1;
        }
        let newline_idx = unsafe { memchr(b'\n', &input[idx..]).unwrap_unchecked() };
        let r =parse_digits(&input[idx..idx + newline_idx]);
        right.entry(r).and_modify(|e| *e += 1).or_insert(1);
        idx = idx + newline_idx + 1;        
    }

    left.into_iter()
        .map(|l| right.get(&l).copied().unwrap_or_default() as u64 * l)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("../input/day1_example.txt")), 11);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(include_str!("../input/day1.txt")), 2769675);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("../input/day1_example.txt")), 31);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(include_str!("../input/day1.txt")), 24643097);
    }
}
use std::{str, sync::LazyLock};

use regex::bytes::{Regex, RegexBuilder};

static PART1_RE: LazyLock<Regex> = LazyLock::new(|| 
    RegexBuilder::new(r"mul\((\d{1,3}),(\d{1,3})\)").unicode(false).build().unwrap()
);

static PART2_RE: LazyLock<Regex> = LazyLock::new(|| 
    RegexBuilder::new(r"((do)\(\)|(don't)\(\)|mul\((\d{1,3}),(\d{1,3})\))").unicode(false).build().unwrap()
);

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    PART1_RE.captures_iter(input).map(|cap| {
        let a: usize = unsafe { str::from_utf8_unchecked(&cap[1]) }.parse().unwrap();
        let b: usize = unsafe { str::from_utf8_unchecked(&cap[2]) }.parse().unwrap();
        a * b
    }).sum()
}

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut do_state = true;
    PART2_RE.captures_iter(input).map(|cap| {
        if cap.get(2).is_some() {
            do_state = true;
            0
        } else if cap.get(3).is_some() {
            do_state = false;
            0
        } else {
            let a: usize = unsafe { str::from_utf8_unchecked(&cap[4]) }.parse().unwrap();
            let b: usize = unsafe { str::from_utf8_unchecked(&cap[5]) }.parse().unwrap();
            if do_state {
                a * b
            } else {
                0
            }
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("../input/day3_example.txt")), 161);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(include_str!("../input/day3.txt")), 179571322);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("../input/day3_example2.txt")), 48);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(include_str!("../input/day3.txt")), 103811193);
    }
}
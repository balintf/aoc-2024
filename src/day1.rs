use memchr::memchr;

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
    let mut left = Vec::<u32>::with_capacity(1000);
    let mut right = Vec::<u32>::with_capacity(1000);
    let mut idx = 0;
    while idx < input.len() {
        let space_idx = unsafe { memchr(b' ', &input[idx..]).unwrap_unchecked() };
        let l = parse_digits(&input[idx..idx + space_idx]) as u32;
        left.push(l);
        idx = idx + space_idx + 1;
        while unsafe { *input.get_unchecked(idx) } == b' ' {
            idx += 1;
        }
        let newline_idx = unsafe { memchr(b'\n', &input[idx..]).unwrap_unchecked() };
        let r =parse_digits(&input[idx..idx + newline_idx]) as u32;
        right.push(r);
        idx = idx + newline_idx + 1;        
    }

    radsort::sort(&mut left);
    radsort::sort(&mut right);
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r) as u64)
        .sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut left = Vec::<u32>::with_capacity(1000);
    let mut counts: Vec<u16> = vec![0; 100000];
    let mut idx = 0;
    while idx < input.len() {
        let space_idx = unsafe { memchr(b' ', &input[idx..]).unwrap_unchecked() };
        let l = parse_digits(&input[idx..idx + space_idx]) as u32;
        left.push(l);
        idx = idx + space_idx + 1;
        while unsafe { *input.get_unchecked(idx) } == b' ' {
            idx += 1;
        }
        let newline_idx = unsafe { memchr(b'\n', &input[idx..]).unwrap_unchecked() };
        let r =parse_digits(&input[idx..idx + newline_idx]);
        unsafe { *counts.get_unchecked_mut(r as usize) += 1; }
        idx = idx + newline_idx + 1;        
    }

    left.into_iter()
        .map(|l| l as u64 * (*unsafe {
            counts.get_unchecked(l as usize)
        } as u64)).sum()
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
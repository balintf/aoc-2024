use memchr::memchr;

#[inline]
fn parse_digits(input: &[u8]) -> u32 {
    let mut result = 0;
    for byte in input {
        result = result * 10 + (byte - b'0') as u32;
    }
    result
}

fn part1_flexible(input: &[u8], digit_len: usize) -> u64 {
    let mut left = Vec::<u32>::with_capacity(1000);
    let mut right = Vec::<u32>::with_capacity(1000);
    for chunk in input.chunks_exact(2*digit_len + 4) {        
        let l = parse_digits(unsafe { chunk.get_unchecked(..digit_len) });
        left.push(l);

        let r =parse_digits(unsafe { chunk.get_unchecked(digit_len + 3..2*digit_len+3) });
        right.push(r);
    }

    radsort::sort(&mut left);
    radsort::sort(&mut right);
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r) as u64)
        .sum::<u64>()
}

#[inline]
fn parse_digits_5(input: &[u8]) -> u32 {
    let mut result = 0;
    for i in 0..5 {
        result = result * 10 + (unsafe { input.get_unchecked(i) } - b'0') as u32;
    }
    result
}

fn part1_assume_length(input: &[u8]) -> u64 {
    let mut left = Vec::<u32>::with_capacity(1000);
    let mut right = Vec::<u32>::with_capacity(1000);
    for chunk in unsafe { input.as_chunks_unchecked::<14>() } {
        let l = parse_digits_5(chunk);
        left.push(l);
        let r = parse_digits_5(&chunk[8..]);
        right.push(r);
    }

    radsort::sort(&mut left);
    radsort::sort(&mut right);
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r) as u64)
        .sum::<u64>()
}

pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    let digit_len = unsafe { memchr(b' ', input).unwrap_unchecked() };
    if digit_len == 5 {
        part1_assume_length(input)
    } else {
        part1_flexible(input, digit_len)
    }
}

fn part2_flexible(input: &[u8], digit_len: usize) -> u64 {
    let mut left = Vec::<u32>::with_capacity(1000);
    let mut counts: Vec<u16> = vec![0; 100000];
    for chunk in input.chunks_exact(2*digit_len + 4) {
        let l = parse_digits(unsafe { chunk.get_unchecked(..digit_len) });
        left.push(l);

        let r =parse_digits(unsafe { chunk.get_unchecked(digit_len + 3..2*digit_len+3) });
        unsafe { *counts.get_unchecked_mut(r as usize) += 1; }
    }

    left.into_iter()
        .map(|l| (l as u64) * (*unsafe {
            counts.get_unchecked(l as usize)
        } as u64)).sum()
}

fn part2_assume_length(input: &[u8]) -> u64 {
    let mut left = Vec::<u32>::with_capacity(1000);
    let mut counts: Vec<u16> = vec![0; 100000];
    for chunk in unsafe { input.as_chunks_unchecked::<14>() } {
        let l = parse_digits_5(chunk);
        left.push(l);

        let r =parse_digits_5(&chunk[8..]);
        unsafe { *counts.get_unchecked_mut(r as usize) += 1; }
    }

    left.into_iter().map(|l| (l as u64) * 
                        (*unsafe { counts.get_unchecked(l as usize) } as u64))
                    .sum()
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let digit_len = unsafe { memchr(b' ', input).unwrap_unchecked() };
    if digit_len == 5 {
        part2_assume_length(input)
    } else {
        part2_flexible(input, digit_len)
    }
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
use memchr::{memchr, memchr_iter};

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut result = 0;
    let mut idx = 0;
    let mut table = [0u8; 10000];
    loop {
        let first_digit = unsafe { *input.get_unchecked(idx) };
        if first_digit == b'\n' {
            idx += 1;
            break;
        }
        let first = (first_digit as usize) *10
            + unsafe { *input.get_unchecked(idx+1) } as usize - b'0' as usize * 11;
        let second = (unsafe { *input.get_unchecked(idx+3) } as usize)*10
            + unsafe { *input.get_unchecked(idx+4) } as usize - b'0' as usize * 11;
        table[first*100+second] = 1;
        idx += 6;
    }
    let mut pages = [0u8; 128];
    while idx < input.len() {
        let line_len = unsafe { memchr(b'\n', &input[idx..]).unwrap_unchecked() };
        let mut num_len = 0;
        let mut pos = idx;
        while pos < idx + line_len {
            let number = unsafe { *input.get_unchecked(pos) } as usize * 10
                + unsafe { *input.get_unchecked(pos+1) } as usize - b'0' as usize * 11;
            pages[num_len] = number as u8;
            num_len += 1;
            pos += 3;
        }
        let mut is_ok = true;
        'outer: for i in 0..num_len {
            let first = pages[i] as usize * 100;
            for j in i+1..num_len {
                if table[first + pages[j] as usize] == 0 {
                    is_ok = false;
                    break 'outer;
                }
            }
        }
        idx += line_len + 1;
        if is_ok {
            result += unsafe { *pages.get_unchecked(num_len/2) } as u32;
        }
    }
    result
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut result = 0;
    let mut idx = 0;
    let mut table = [0u8; 10000];
    loop {
        let first_digit = unsafe { *input.get_unchecked(idx) };
        if first_digit == b'\n' {
            idx += 1;
            break;
        }
        let first = (first_digit as usize) *10
            + unsafe { *input.get_unchecked(idx+1) } as usize - b'0' as usize * 11;
        let second = (unsafe { *input.get_unchecked(idx+3) } as usize)*10
            + unsafe { *input.get_unchecked(idx+4) } as usize - b'0' as usize * 11;
        table[first*100+second] = 1;
        idx += 6;
    }
    let mut pages = [0u8; 128];
    while idx < input.len() {
        let line_len = unsafe { memchr(b'\n', &input[idx..]).unwrap_unchecked() };
        let mut num_len = 0;
        let mut pos = idx;
        while pos < idx + line_len {
            let number = unsafe { *input.get_unchecked(pos) } as usize * 10
                + unsafe { *input.get_unchecked(pos+1) } as usize - b'0' as usize * 11;
            pages[num_len] = number as u8;
            num_len += 1;
            pos += 3;
        }
        let mut is_ok = true;
        for i in 0..num_len {
            let mut first = pages[i] as usize * 100;
            let mut j = i+1;
            while j < num_len {
                if table[first + pages[j] as usize] == 0 {
                    is_ok = false;
                    first = pages[j] as usize * 100;
                    pages.swap(i, j);
                    j = i+1;
                } else {
                    j += 1;
                }
            }
        }
        idx += line_len + 1;
        if !is_ok {
            result += unsafe { *pages.get_unchecked(num_len/2) } as u32;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("../input/day5_example.txt")), 143);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(include_str!("../input/day5.txt")), 5166);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("../input/day5_example.txt")), 123);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(include_str!("../input/day5.txt")), 4679);
    }
}
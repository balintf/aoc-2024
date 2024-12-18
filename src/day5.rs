use std::mem::{MaybeUninit, transmute};

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut result = 0;
    let mut idx = 0;
    let mut table = [0u8; 128*128];
    loop {
        let first_digit = unsafe { *input.get_unchecked(idx) };
        if first_digit == b'\n' {
            idx += 1;
            break;
        }
        let table_idx = (first_digit as u32) *1280
            + (unsafe { *input.get_unchecked(idx+1) } as u32)*128
            + (unsafe { *input.get_unchecked(idx+3) } as u32)*10
            + unsafe { *input.get_unchecked(idx+4) } as u32 - b'0' as u32 * (1280+128+10+1);
        *unsafe { table.get_unchecked_mut(table_idx as usize) } = 1;
        idx += 6;
    }
    let mut pages = [const { MaybeUninit::<u8>::uninit()}; 128];
    while idx < input.len() {
        let mut num_len = 0;
        loop {
            let number = unsafe { *input.get_unchecked(idx) } as u32 * 10
                       + unsafe { *input.get_unchecked(idx+1) } as u32
                       - b'0' as u32 * 11;
            let end = unsafe { *input.get_unchecked(idx+2) };
            unsafe { pages.get_unchecked_mut(num_len).write(number as u8) };
            num_len += 1;
            idx += 3;
            if end == b'\n' {
                break;
            }
        }

        let pages: &[u8] = unsafe { transmute(pages.get_unchecked(..num_len)) };
        if pages.is_sorted_by(|a, b| 
            unsafe { *table.get_unchecked((((*a as u32) << 7) + *b as u32) as usize) } == 1
        ) {
            result += unsafe { *pages.get_unchecked(num_len/2) } as u32;
        }
    }
    result
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut result = 0;
    let mut idx = 0;
    let mut table = [0u8; 128*128];
    loop {
        let first_digit = unsafe { *input.get_unchecked(idx) };
        if first_digit == b'\n' {
            idx += 1;
            break;
        }
        let table_idx = (first_digit as u32) *1280
            + (unsafe { *input.get_unchecked(idx+1) } as u32)*128
            + (unsafe { *input.get_unchecked(idx+3) } as u32)*10
            + unsafe { *input.get_unchecked(idx+4) } as u32 - b'0' as u32 * (1280+128+10+1);
        *unsafe { table.get_unchecked_mut(table_idx as usize) } = 1;
        idx += 6;
    }
    let mut pages = [const { MaybeUninit::<u8>::uninit() }; 128];
    while idx < input.len() {
        let mut num_len = 0;
        loop {
            let number = unsafe { *input.get_unchecked(idx) } as u32 * 10
                       + unsafe { *input.get_unchecked(idx+1) } as u32
                       - b'0' as u32 * 11;
            let end = unsafe { *input.get_unchecked(idx+2) };
            unsafe { pages.get_unchecked_mut(num_len).write(number as u8) };
            num_len += 1;
            idx += 3;
            if end == b'\n' {
                break;
            }
        }

        let pages: &mut [u8] = unsafe { transmute(pages.get_unchecked_mut(..num_len)) };
        if !pages.is_sorted_by(|a, b| unsafe { *table.get_unchecked((((*a as u32) << 7) + *b as u32) as usize) } == 1) {
            let (_, x, _) = pages.select_nth_unstable_by(num_len/2, |a, b| {
                if *a == *b {
                    std::cmp::Ordering::Equal
                } else if unsafe { *table.get_unchecked((((*a as u32) << 7) + *b as u32) as usize) } == 1 {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            result += *x as u32;
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

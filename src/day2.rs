use std::{hint::assert_unchecked, mem::MaybeUninit};

#[inline]
fn parse_line<'a>(input: &[u8], index: &mut usize, data_buffer: &'a mut [MaybeUninit<i32>; 16]) -> &'a mut [i32] {
    let mut len = 0;
    let mut x = 0;
    let mut current = unsafe { *input.get_unchecked(*index) };
    *index += 1;
    loop {
        unsafe { assert_unchecked(len < 16); }
        match current {
            b'\n' => {
                unsafe { data_buffer.get_unchecked_mut(len).as_mut_ptr().write(x) };
                len += 1;
                break;
            },
            b' ' => {
                unsafe { data_buffer.get_unchecked_mut(len).as_mut_ptr().write(x) };
                len += 1;
                x = 0;
            }
            _ => { 
                x = x * 10 + (current - b'0') as i32;
            }
        }
        current = unsafe { *input.get_unchecked(*index) };
        *index += 1;
    }
    unsafe { std::mem::transmute(data_buffer.get_unchecked_mut(..len)) }
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut result = 0;
    let mut data_buffer = [const { MaybeUninit::<i32>::uninit() }; 16];
    let mut index = 0;
    while index < input.len() {
        let data = parse_line(input, &mut index, &mut data_buffer);
        unsafe {
            assert_unchecked(data.len() >= 2);
        }
        let first_diff = data[1] - data[0];
        if first_diff == 0 || first_diff.abs() > 3 {
            continue;
        }
        let mut is_ok = true;
        if first_diff > 0 {
            for i in 2..data.len() {
                let diff = data[i] - data[i-1];
                if diff <= 0 || diff > 3 {
                    is_ok = false;
                    break;
                }
            }
        } else {
            for i in 2..data.len() {
                let diff = data[i] - data[i-1];
                if diff >= 0 || diff < -3 {
                    is_ok = false;
                    break;
                }
            }
        }

        if is_ok {
            result += 1;
        }
    }
    result
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut result = 0;
    let mut data_buffer = [const { MaybeUninit::<i32>::uninit() }; 16];
    let mut index = 0;
    while index < input.len() {
        let data = parse_line(input, &mut index, &mut data_buffer);
        unsafe {
            assert_unchecked(data.len() >= 5);
        }
        for i in 0..data.len() {
            let mut pos_diffs = 0;
            let mut neg_diffs = 0;
            let mut is_ok = true;
            for j in 0..data.len()-1 {
                let mut x = data[j];
                let mut y = data[j+1];
                if i == j  {
                    if j == 0 {
                        continue;
                    }
                    x = unsafe { *data.get_unchecked(j-1) };
                } else if i == j+1 {
                    if j + 2 == data.len() {
                        continue;
                    }
                    y = unsafe { *data.get_unchecked(j+2) };
                }
                let diff = y - x;
                if diff > 0  && diff < 4 {
                    pos_diffs += 1;
                } else if diff < 0 && diff > -4 {
                    neg_diffs += 1;
                } else {
                    is_ok = false;
                    break;
                }
            }
            if is_ok && (pos_diffs == 0 || neg_diffs == 0) {
                result += 1;
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("../input/day2_example.txt")), 2);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(include_str!("../input/day2.txt")), 369);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("../input/day2_example.txt")), 4);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(include_str!("../input/day2.txt")), 428);
    }
}

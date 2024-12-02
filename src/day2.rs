use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    let mut result = 0;
    let mut data = Vec::with_capacity(16);
    for line in input.lines() {
        let mut is_ok = true;
        data.clear();
        data.extend(line.split_ascii_whitespace().map(|x| unsafe { x.parse::<i32>().unwrap_unchecked() }));
        let first_diff = unsafe { *data.get_unchecked(1) - *data.get_unchecked(0) };
        if first_diff == 0 || first_diff.abs() > 3 {
            continue;
        }
        if first_diff > 0 {
            for (x, y) in unsafe { data.get_unchecked(1..) }.iter().tuple_windows() {
                let diff = y - x;
                if diff <= 0 || diff > 3 {
                    is_ok = false;
                    break;
                }
            }
        } else {
            for (x, y) in unsafe { data.get_unchecked(1..) }.iter().tuple_windows() {
                let diff = y - x;
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
    let mut result = 0;
    let mut data = Vec::with_capacity(16);
    for line in input.lines() {
        data.clear();
        data.extend(line.split_ascii_whitespace().map(|x| unsafe { x.parse::<i32>().unwrap_unchecked() }));
        for i in 0..data.len() {
            let mut pos_diffs = 0;
            let mut neg_diffs = 0;
            let mut is_ok = true;
            for (x, y) in data.iter().enumerate().filter_map(|(j, d)| if i == j { None } else { Some(*d) }).tuple_windows() {
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

use memchr::memchr;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub fn part1(input: &str) -> u32 {
    let mut input = input.as_bytes().to_vec();
    let mut result = 1;
    let line_len = (unsafe { memchr(b'\n', &input).unwrap_unchecked() } + 1) as isize;
    let start_pos = unsafe { memchr(b'^', &input).unwrap_unchecked() } as isize;

    let mut x: isize = start_pos % line_len;
    let mut y: isize = start_pos / line_len;
    let mut dir = Direction::Up;
    loop {
        if input[(y * line_len + x) as usize] == b'.' {
            input[(y * line_len + x) as usize] = b'X';
            result += 1;
        }
        let (orig_x, orig_y) = (x, y);
        match dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1
        }
        if x >= 0 && x < line_len-1 && y >= 0 && y < input.len() as isize / line_len {
            if input[(y * line_len + x) as usize] == b'#' {
                x = orig_x;
                y = orig_y;
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up
                };
            }
        } else {
            break;
        }
    }

    result
}

pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("../input/day6_example.txt")), 41);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(include_str!("../input/day6.txt")), 4433);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("../input/day6_example.txt")), 6);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(include_str!("../input/day6.txt")), 4679);
    }
}
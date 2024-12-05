use memchr::memchr;

#[inline]
fn vert_match(input: &[u8], pos: usize, line_len: usize) -> bool {
    if pos + 3*line_len+3 < input.len()
       && input[pos + line_len + 1] == b'M'
       && input[pos + 2*line_len + 2] == b'A'
       && input[pos + 3*line_len + 3] == b'S'
    {
        return true;
    }
    false
}

#[inline]
fn vert_match_inverted(input: &[u8], pos: usize, line_len: usize) -> bool {
    if pos >= 3*line_len-3
       && input[pos - line_len-1] == b'M'
       && input[pos - 2*line_len-2] == b'A'
       && input[pos - 3*line_len-3] == b'S'
    {
        return true;
    }
    false
}

#[inline]
fn diag_match_rd(input: &[u8], pos: usize, line_len: usize) -> bool {
    if pos + 3*line_len+6 < input.len()
       && input[pos + line_len + 2] == b'M'
       && input[pos + 2*line_len + 4] == b'A'
       && input[pos + 3*line_len + 6] == b'S'
    {
        return true;
    }
    false
}

#[inline]
fn diag_match_ld(input: &[u8], pos: usize, line_len: usize) -> bool {
    if pos + 3*line_len-4 < input.len()
       && input[pos + line_len] == b'M'
       && input[pos + 2*line_len] == b'A'
       && input[pos + 3*line_len] == b'S'
    {
        return true;
    }
    false
}

#[inline]
fn diag_match_ru(input: &[u8], pos: usize, line_len: usize) -> bool {
    if pos >= 3*line_len
       && input[pos - line_len] == b'M'
       && input[pos - 2*line_len] == b'A'
       && input[pos - 3*line_len] == b'S'
    {
        return true;
    }
    false
}

#[inline]
fn diag_match_lu(input: &[u8], pos: usize, line_len: usize) -> bool {
    if pos >= 3*line_len+6
       && input[pos - line_len - 2] == b'M'
       && input[pos - 2*line_len - 4] == b'A'
       && input[pos - 3*line_len - 6] == b'S'
    {
        return true;
    }
    false
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let line_len = unsafe { memchr(b'\n', input).unwrap_unchecked() };
    let mut result = 0;
    for pos in memchr::memchr_iter(b'X', input) {
        if pos+3 < input.len() && &input[pos..pos+4] == b"XMAS" {
            result += 1;
        }
        if pos >= 3 && &input[pos-3..pos+1] == b"SAMX" {
            result += 1;
        }
        if vert_match(input, pos, line_len) {
            result += 1;
        }
        if vert_match_inverted(input, pos, line_len) {
            result += 1;
        }
        if diag_match_rd(input, pos, line_len) {
            result += 1;
        }
        if diag_match_ld(input, pos, line_len) {
            result += 1;
        }
        if diag_match_ru(input, pos, line_len) {
            result += 1;
        }
        if diag_match_lu(input, pos, line_len) {
            result += 1;
        }
    }
    result
}

const PATTERNS: [[u8; 4]; 4] = [
    [b'M', b'S', b'M', b'S'],
    [b'M', b'M', b'S', b'S'],
    [b'S', b'M', b'S', b'M'],
    [b'S', b'S', b'M', b'M'],
];

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let line_len = unsafe { memchr(b'\n', input).unwrap_unchecked() };
    let mut result = 0;
    for pos in memchr::memchr_iter(b'A', input) {
        if pos >= line_len + 2 && pos + line_len + 2 < input.len() {
            let corners: [u8; 4] = [
                input[pos - line_len - 2],
                input[pos - line_len],
                input[pos + line_len],
                input[pos + line_len + 2],
            ];
            for pattern in PATTERNS {
                if pattern == corners {
                    result += 1;
                    break;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_horizontal() {
        let input = concat!(
                    "XMAS\n",
                    "SAMX\n",
                    "MASX\n",
                    "XMAS\n");
        println!("{}", input);
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn part1_vertical() {
        let input = concat!(
            "XMAS\n",
            "M..A\n",
            "A..M\n",
            "S..X\n");
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn part1_diagonal() {
        let input = concat!(
            "X..S\n",
            ".MA.\n",
            ".MA.\n",
            "X..S\n");
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part1_diagonal_inv() {
        let input = concat!(
            "S..X\n",
            ".AM.\n",
            ".AM.\n",
            "S..X\n");
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("../input/day4_example.txt")), 18);
    }

    #[test]
    fn part1_solution() {
        assert_eq!(part1(include_str!("../input/day4.txt")), 2500);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("../input/day4_example.txt")), 9);
    }

    #[test]
    fn part2_solution() {
        assert_eq!(part2(include_str!("../input/day4.txt")), 1933);
    }
}

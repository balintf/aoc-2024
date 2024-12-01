use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let mut left = Vec::<i32>::with_capacity(1000);
    let mut right = Vec::<i32>::with_capacity(1000);
    input.lines().for_each(|line| {
        let (l, r) = line.split_at(line.find(' ').unwrap());
        let r = r.trim_start();

        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
    });
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut left = Vec::<i32>::with_capacity(1000);
    let mut right = HashMap::<i32, i32>::with_capacity(1000);
    input.lines().for_each(|line| {
        let (l, r) = line.split_at(line.find(' ').unwrap());
        let r = r.trim_start();

        left.push(l.parse::<i32>().unwrap());
        let right_val = r.parse::<i32>().unwrap();
        right.entry(right_val).and_modify(|e| *e += 1).or_insert(1);
    });

    left.into_iter()
        .map(|l| right.get(&l).map(|&count| l * count).unwrap_or(0) as u32)
        .sum()
}

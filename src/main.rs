fn main() {
    let day1_input = include_str!("../input/day1.txt");
    let day1_part1 = aoc_2024::day1::part1(day1_input);
    println!("Day 1 part 1: {}", day1_part1);
    let day1_part2 = aoc_2024::day1::part2(day1_input);
    println!("Day 1 part 2: {}", day1_part2);
}

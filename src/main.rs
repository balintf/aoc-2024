macro_rules! run_day {
    ($day:ident) => {
        let day_input = include_str!(concat!("../input/", stringify!($day), ".txt"));
        let example_input = include_str!(concat!("../input/", stringify!($day), "_example.txt"));
        let day_part1_example = aoc_2024::$day::part1(example_input);
        println!("Day {} example part 1: {}", stringify!($day), day_part1_example);
        let day_part2_example = aoc_2024::$day::part2(example_input);
        println!("Day {} example part 2: {}", stringify!($day), day_part2_example);
        let day_part1 = aoc_2024::$day::part1(day_input);
        println!("Day {} part 1: {}", stringify!($day), day_part1);
        let day_part2 = aoc_2024::$day::part2(day_input);
        println!("Day {} part 2: {}", stringify!($day), day_part2);
    };
}

fn main() {
    run_day!(day1);
    run_day!(day2);
    run_day!(day3);
    run_day!(day4);
}

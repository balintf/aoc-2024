use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_day1_part1(c: &mut Criterion) {
    let input = include_str!("../input/day1.txt");
    c.bench_function("day1 part1", |b| {
        b.iter(|| aoc_2024::day1::part1(black_box(input)))
    });
}

fn bench_day1_part2(c: &mut Criterion) {
    let input = include_str!("../input/day1.txt");
    c.bench_function("day1 part2", |b| {
        b.iter(|| aoc_2024::day1::part2(black_box(input)))
    });
}

fn bench_day2_part1(c: &mut Criterion) {
    let input = include_str!("../input/day2.txt");
    c.bench_function("day2 part1", |b| {
        b.iter(|| aoc_2024::day2::part1(black_box(input)))
    });
}

fn bench_day2_part2(c: &mut Criterion) {
    let input = include_str!("../input/day2.txt");
    c.bench_function("day2 part2", |b| {
        b.iter(|| aoc_2024::day2::part2(black_box(input)))
    });
}

fn bench_day3_part1(c: &mut Criterion) {
    let input = include_str!("../input/day3.txt");
    c.bench_function("day3 part1", |b| {
        b.iter(|| aoc_2024::day3::part1(black_box(input)))
    });
}

fn bench_day3_part2(c: &mut Criterion) {
    let input = include_str!("../input/day3.txt");
    c.bench_function("day3 part2", |b| {
        b.iter(|| aoc_2024::day3::part2(black_box(input)))
    });
}

fn bench_day4_part1(c: &mut Criterion) {
    let input = include_str!("../input/day4.txt");
    c.bench_function("day4 part1", |b| {
        b.iter(|| aoc_2024::day4::part1(black_box(input)))
    });
}

fn bench_day4_part2(c: &mut Criterion) {
    let input = include_str!("../input/day4.txt");
    c.bench_function("day4 part2", |b| {
        b.iter(|| aoc_2024::day4::part2(black_box(input)))
    });
}

fn bench_day5_part1(c: &mut Criterion) {
    let input = include_str!("../input/day5.txt");
    c.bench_function("day5 part1", |b| {
        b.iter(|| aoc_2024::day5::part1(black_box(input)))
    });
}

fn bench_day5_part2(c: &mut Criterion) {
    let input = include_str!("../input/day5.txt");
    c.bench_function("day5 part2", |b| {
        b.iter(|| aoc_2024::day5::part2(black_box(input)))
    });
}


criterion_group!(day1_benches, bench_day1_part1, bench_day1_part2);
criterion_group!(day2_benches, bench_day2_part1, bench_day2_part2);
criterion_group!(day3_benches, bench_day3_part1, bench_day3_part2);
criterion_group!(day4_benches, bench_day4_part1, bench_day4_part2);
criterion_group!(day5_benches, bench_day5_part1, bench_day5_part2);
criterion_main!(day1_benches, day2_benches, day3_benches, day4_benches, day5_benches);

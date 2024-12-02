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

criterion_group!(day1_benches, bench_day1_part1, bench_day1_part2);
criterion_group!(day2_benches, bench_day2_part1, bench_day2_part2);
criterion_main!(day1_benches, day2_benches);

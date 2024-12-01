use std::collections::HashMap;

// Scores:
// day1::part1 (avg 34.347µs, stddev 5.150µs in 14558 runs)
// day1::part2 (avg 52.756µs, stddev 5.080µs in 9478 runs)

fn parse_input(input: &str) -> impl Iterator<Item = (isize, isize)> + '_ {
    input.lines().map(|line| {
        let a = &line[0..5];
        let b = &line[8..];
        (
            a.parse::<isize>().expect("parse int"),
            b.parse::<isize>().expect("parse int"),
        )
    })
}

pub fn part1(input: &str) -> isize {
    let (mut list1, mut list2): (Vec<isize>, Vec<isize>) = parse_input(input).unzip();
    list1.sort_unstable();
    list2.sort_unstable();

    let sum: isize = list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| (a - b).abs())
        .sum();
    sum
}

pub fn part2(input: &str) -> isize {
    let (list1, list2): (Vec<isize>, Vec<isize>) = parse_input(input).unzip();
    let mut appearances = HashMap::new();

    for value in list2 {
        let entry = appearances.entry(value).or_insert(0);
        *entry += 1;
    }

    let sum: isize = list1
        .into_iter()
        .map(|v| v * *appearances.get(&v).unwrap_or(&0))
        .sum();
    sum
}

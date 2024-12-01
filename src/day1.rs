use std::collections::BTreeMap;

// Scores:
// day1::part1 (avg 53.765µs, stddev 7.030µs in 9300 runs)
// day1::part2 (avg 102.831µs, stddev 10.562µs in 4863 runs)

fn parse_input(input: &str) -> (Vec<isize>, Vec<isize>) {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let Some(a) = iter.next() else {
                panic!("invalid line")
            };
            let Some(b) = iter.next() else {
                panic!("invalid line")
            };
            (
                a.parse::<isize>().expect("parse int"),
                b.parse::<isize>().expect("parse int"),
            )
        })
        .unzip()
}

pub fn part1(input: &str) -> isize {
    let (mut list1, mut list2) = parse_input(input);

    list1.sort_unstable();
    list2.sort_unstable();
    let sum: isize = list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    sum
}

pub fn part2(input: &str) -> isize {
    let (list1, list2) = parse_input(input);
    let mut appearances = BTreeMap::new();
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

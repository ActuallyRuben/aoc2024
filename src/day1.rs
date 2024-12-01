use std::collections::BTreeMap;

const INPUT: &str = include_str!("input/day1.txt");

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

pub fn part1() {
    let (mut list1, mut list2) = parse_input(INPUT);

    list1.sort_unstable();
    list2.sort_unstable();
    let sum: isize = list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    println!("{sum}");
}

pub fn part2() {
    let input = include_str!("input/day1.txt");

    let (list1, list2) = parse_input(INPUT);
    let mut appearances = BTreeMap::new();
    for value in list2 {
        let entry = appearances.entry(value).or_insert(0);
        *entry += 1;
    }

    let sum: isize = list1
        .into_iter()
        .map(|v| v * *appearances.get(&v).unwrap_or(&0))
        .sum();
    println!("{sum}")
}

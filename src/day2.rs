#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Ascending,
    Descending,
}

fn get_direction(diff: isize) -> Option<Direction> {
    match diff {
        1..=3 => Some(Direction::Ascending),
        -3..=-1 => Some(Direction::Descending),
        _ => None,
    }
}

pub fn is_safe(report: &[isize]) -> bool {
    let Some(direction) = get_direction(report[1] - report[0]) else {
        return false;
    };
    for r in report[1..].windows(2) {
        let [prev, x] = r else { unreachable!() };
        if Some(direction) != get_direction(*x - *prev) {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|x| {
            let report = x
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            if is_safe(&report) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn is_damped_safe(report: &[isize]) -> bool {
    let mut damped_report = vec![0; report.len() - 1];
    for i in 0..report.len() {
        damped_report[0..i].clone_from_slice(&report[0..i]);
        damped_report[i..].clone_from_slice(&report[i + 1..]);
        if is_safe(&damped_report) {
            return true;
        }
    }
    false
}

pub fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|x| {
            let report = x
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            if is_safe(&report) || is_damped_safe(&report) {
                1
            } else {
                0
            }
        })
        .sum()
}

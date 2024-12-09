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

pub fn is_safe(mut report: impl Iterator<Item = isize>) -> bool {
    let first = report.next().unwrap();
    let second = report.next().unwrap();
    let Some(direction) = get_direction(second - first) else {
        return false;
    };
    let mut prev = second;
    for next in report {
        if Some(direction) != get_direction(next - prev) {
            return false;
        }
        prev = next;
    }
    true
}

pub fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|x| {
            let report = x.split_whitespace().map(|x| x.parse::<isize>().unwrap());
            if is_safe(report) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn is_damped_safe(report: impl ExactSizeIterator<Item = isize> + Clone) -> bool {
    for i in 0..report.len() {
        let damped_report = report.clone().take(i).chain(report.clone().skip(i + 1));
        if is_safe(damped_report) {
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
            if is_safe(report.iter().copied()) || is_damped_safe(report.iter().copied()) {
                1
            } else {
                0
            }
        })
        .sum()
}

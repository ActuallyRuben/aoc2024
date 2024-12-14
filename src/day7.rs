use rayon::prelude::*;

fn parse_line(line: &str) -> (usize, Vec<usize>) {
    let mut spliterator = line.splitn(2, ": ");
    let result: usize = spliterator.next().unwrap().parse().unwrap();
    let seq: Vec<usize> = spliterator
        .next()
        .unwrap()
        .split_whitespace()
        .rev()
        .map(|x| x.parse().unwrap())
        .collect();
    (result, seq)
}

fn can_fit(total: usize, values: &[usize]) -> bool {
    let head = values[0];
    if values.len() == 1 {
        return total == head;
    }
    (total % head == 0 && can_fit(total / head, &values[1..]))
        || (total > head && can_fit(total - head, &values[1..]))
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let (result, seq) = parse_line(line);
            if can_fit(result, &seq) {
                result
            } else {
                0
            }
        })
        .sum()
}

fn deconcat(rhs: usize, result: usize) -> Option<usize> {
    if rhs >= result {
        return None;
    }
    let lhs_mulled = result - rhs;
    let mut rhs_amt = 1;

    while rhs_amt <= rhs {
        rhs_amt *= 10;
    }

    if lhs_mulled % rhs_amt == 0 {
        Some(lhs_mulled / rhs_amt)
    } else {
        None
    }
}

fn can_fit_concat(total: usize, values: &[usize]) -> bool {
    let head = values[0];
    if values.len() == 1 {
        return total == head;
    }
    (total % head == 0 && can_fit_concat(total / head, &values[1..]))
        || (total > head && can_fit_concat(total - head, &values[1..]))
        || deconcat(head, total).is_some_and(|lhs| can_fit_concat(lhs, &values[1..]))
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let (result, seq) = parse_line(line);
            if can_fit_concat(result, &seq) {
                result
            } else {
                0
            }
        })
        .sum()
}

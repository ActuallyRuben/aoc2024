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
    if total % head == 0 && can_fit(total / head, &values[1..]) {
        true
    } else if total > head && can_fit(total - head, &values[1..]) {
        true
    } else {
        false
    }
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
    let rhs_digits = (rhs as f32).log10().floor() as u32 + 1;
    let lhs_mulled = result - rhs;
    let rhs_amt = 10usize.pow(rhs_digits);
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
    if total % head == 0 && can_fit_concat(total / head, &values[1..]) {
        true
    } else if total > head && can_fit_concat(total - head, &values[1..]) {
        true
    } else if deconcat(head, total).is_some_and(|lhs| can_fit_concat(lhs, &values[1..])) {
        true
    } else {
        false
    }
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

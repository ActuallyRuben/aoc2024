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
        .map(|line| {
            let mut spliterator = line.splitn(2, ": ");
            let result: usize = spliterator.next().unwrap().parse().unwrap();
            let seq: Vec<usize> = spliterator
                .next()
                .unwrap()
                .split_whitespace()
                .rev()
                .map(|x| x.parse().unwrap())
                .collect();
            if can_fit(result, &seq) {
                result
            } else {
                0
            }
        })
        .sum()
}

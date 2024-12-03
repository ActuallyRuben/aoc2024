pub fn part1(mut input: &str) -> usize {
    let mut total = 0;
    while !input.is_empty() {
        if !input.starts_with("mul(") {
            input = &input[1..];
            continue;
        }
        input = &input[4..];
        let mut iter = input.splitn(2, ",");
        let Some(x) = iter.next().and_then(|x| x.parse::<usize>().ok()) else {
            continue;
        };
        input = iter.next().unwrap();
        let mut iter = input.splitn(2, ")");
        let Some(y) = iter.next().and_then(|x| x.parse::<usize>().ok()) else {
            continue;
        };
        input = iter.next().unwrap();
        total += x * y;
    }
    total
}

pub fn part2(mut input: &str) -> usize {
    let mut total = 0;
    let mut enabled = true;
    while !input.is_empty() {
        if input.starts_with("do") {
            if input[2..].starts_with("n") {
                // don't()
                input = &input[6..];
                enabled = false;
                continue;
            } else {
                // do()
                input = &input[3..];
                enabled = true;
                continue;
            }
        }
        if !enabled || !input.starts_with("mul(") {
            input = &input[1..];
            continue;
        }
        input = &input[4..];
        let mut iter = input.splitn(2, ",");
        let Some(x) = iter.next().and_then(|x| x.parse::<usize>().ok()) else {
            continue;
        };
        input = iter.next().unwrap();
        let mut iter = input.splitn(2, ")");
        let Some(y) = iter.next().and_then(|x| x.parse::<usize>().ok()) else {
            continue;
        };
        input = iter.next().unwrap();
        if enabled {
            total += x * y;
        }
    }
    total
}

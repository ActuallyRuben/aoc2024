fn split_number(input: &str, sep: char) -> Option<(usize, &str)> {
    for (i, c) in input.chars().enumerate() {
        match c {
            '0'..='9' => continue,
            c if i > 0 && c == sep => return Some((input[0..i].parse().unwrap(), &input[i + 1..])),
            _ => return None,
        }
    }
    None
}

pub fn part1(mut input: &str) -> usize {
    let mut total = 0;
    while !input.is_empty() {
        if !input.starts_with("mu") {
            input = &input[1..];
            continue;
        }
        input = &input[4..];
        let Some((x, next)) = split_number(input, ',') else {
            continue;
        };
        input = next;
        let Some((y, next)) = split_number(input, ')') else {
            continue;
        };
        input = next;
        total += x * y;
    }
    total
}

pub fn part2(mut input: &str) -> usize {
    let mut total = 0;
    let mut enabled = true;
    while !input.is_empty() {
        if input.starts_with("do") {
            if input.starts_with("don'") {
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

        if !enabled || !input.starts_with("mu") {
            input = &input[1..];
            continue;
        }
        input = &input[4..];
        let Some((x, next)) = split_number(input, ',') else {
            continue;
        };
        input = next;
        let Some((y, next)) = split_number(input, ')') else {
            continue;
        };
        input = next;
        total += x * y;
    }
    total
}

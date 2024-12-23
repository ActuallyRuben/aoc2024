use crate::util::count_digits;
use std::collections::HashMap;

fn count_rocks(value: usize, blinks_left: usize, cache: &mut [HashMap<usize, usize>]) -> usize {
    if blinks_left == 0 {
        return 1;
    }
    if let Some(result) = cache[blinks_left - 1].get(&value) {
        return *result;
    }
    let digits = count_digits(value) as u32;
    let result = if value == 0 {
        count_rocks(1, blinks_left - 1, cache)
    } else if digits % 2 == 0 {
        let digit_mask = 10usize.pow(digits / 2);
        count_rocks(value / digit_mask, blinks_left - 1, cache)
            + count_rocks(value % digit_mask, blinks_left - 1, cache)
    } else {
        count_rocks(value * 2024, blinks_left - 1, cache)
    };
    cache[blinks_left - 1].insert(value, result);
    result
}

pub fn part1(input: &str) -> usize {
    let mut cache = [(); 25].map(|_| HashMap::new());
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    stones
        .iter()
        .map(|rock| count_rocks(*rock, 25, &mut cache))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut cache = [(); 75].map(|_| HashMap::new());
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    stones
        .iter()
        .map(|rock| count_rocks(*rock, 75, &mut cache))
        .sum()
}

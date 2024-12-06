use std::collections::{HashMap, HashSet};

fn print_valid(rels: &HashMap<usize, HashSet<usize>>, seq: &[usize]) -> bool {
    let empty_set = HashSet::new();
    for (i, x) in seq.iter().enumerate() {
        let afterset = rels.get(x).unwrap_or(&empty_set);
        for y in &seq[0..i] {
            if afterset.contains(y) {
                return false;
            }
        }
    }
    true
}

fn fix_invalid(rels: &HashMap<usize, HashSet<usize>>, seq: &mut [usize]) -> bool {
    let empty_set = HashSet::new();
    let mut any_fixes = false;
    for i in 0..seq.len() {
        let (seq, tail) = seq.split_at_mut(i);
        let x = &mut tail[0];
        let afterset = rels.get(x).unwrap_or(&empty_set);
        for y in &mut seq[0..i] {
            if afterset.contains(y) {
                std::mem::swap(x, y);
                any_fixes = true;
            }
        }
    }
    any_fixes
}

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut after_rels = HashMap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut split = line.splitn(2, '|');
        let before = split.next().unwrap().parse::<usize>().unwrap();
        let after = split.next().unwrap().parse::<usize>().unwrap();
        let afterset = after_rels.entry(before).or_insert_with(HashSet::new);
        afterset.insert(after);
    }

    lines
        .map(|line| {
            let values: Vec<usize> = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            if print_valid(&after_rels, &values) {
                values[values.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut after_rels = HashMap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut split = line.splitn(2, '|');
        let before = split.next().unwrap().parse::<usize>().unwrap();
        let after = split.next().unwrap().parse::<usize>().unwrap();
        let afterset = after_rels.entry(before).or_insert_with(HashSet::new);
        afterset.insert(after);
    }

    lines
        .map(|line| {
            let mut values: Vec<usize> = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            if fix_invalid(&after_rels, &mut values) {
                values[values.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

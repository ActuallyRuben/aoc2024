use std::collections::HashMap;

fn can_make_pattern(pattern: &[u8], towels: &[&[u8]]) -> bool {
    if pattern.is_empty() {
        return true;
    }
    for towel in towels {
        if towel.len() <= pattern.len()
            && **towel == pattern[0..towel.len()]
            && can_make_pattern(&pattern[towel.len()..], towels)
        {
            return true;
        }
    }
    false
}

pub fn part1(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels: Vec<&[u8]> = towels.split(", ").map(|x| x.as_bytes()).collect();
    let patterns = patterns.lines().map(|pat| pat.as_bytes());
    patterns
        .filter(|pat| can_make_pattern(pat, &towels))
        .count()
}

fn pattern_options<'a>(
    cache: &mut HashMap<&'a [u8], usize>,
    pattern: &'a [u8],
    towels: &[&[u8]],
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(v) = cache.get(pattern) {
        return *v;
    }
    let mut count = 0;
    for towel in towels {
        if towel.len() <= pattern.len() && **towel == pattern[0..towel.len()] {
            count += pattern_options(cache, &pattern[towel.len()..], towels);
        }
    }
    cache.insert(pattern, count);
    count
}

pub fn part2(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels: Vec<&[u8]> = towels.split(", ").map(|x| x.as_bytes()).collect();
    let patterns = patterns.lines().map(|pat| pat.as_bytes());
    let mut cache = HashMap::new();
    patterns
        .map(|pat| pattern_options(&mut cache, pat, &towels))
        .sum()
}

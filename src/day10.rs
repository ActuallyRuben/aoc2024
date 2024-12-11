use crate::util::{Direction, Grid, RefGrid};
use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> usize {
    let chart = RefGrid::from_str(input);
    let mut grid: Grid<HashSet<(usize, usize)>> = Grid::new(chart.width(), chart.height());

    for (pos, val) in chart.iter() {
        if *val == b'9' {
            grid[pos].insert(pos);
        }
    }
    for c in (b'0'..=b'8').rev() {
        'inner: for (pos, val) in chart.iter() {
            if c != *val {
                continue 'inner;
            }
            let positions: HashSet<_> = Direction::iter()
                .filter(|dir| grid.contains(pos + *dir) && chart[pos + *dir] == (*val + 1))
                .flat_map(|dir| grid[pos + dir].iter().copied())
                .collect();
            grid[pos] = positions;
        }
    }

    chart
        .iter()
        .zip(grid.iter())
        .filter(|((_, c), _)| **c == b'0')
        .map(|(_, (_, v))| v.len())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let chart = RefGrid::from_str(input);
    let mut grid: Grid<HashMap<(usize, usize), usize>> = Grid::new(chart.width(), chart.height());

    for (pos, val) in chart.iter() {
        if *val == b'9' {
            grid[pos].insert(pos, 1);
        }
    }
    for c in (b'0'..=b'8').rev() {
        'inner: for (pos, val) in chart.iter() {
            if c != *val {
                continue 'inner;
            }
            let mut positions = HashMap::new();
            for (pos, count) in Direction::iter()
                .filter(|dir| grid.contains(pos + *dir) && chart[pos + *dir] == (*val + 1))
                .flat_map(|dir| grid[pos + dir].iter())
            {
                let entry = positions.entry(*pos).or_insert_with(|| 0);
                *entry += *count;
            }
            grid[pos] = positions;
        }
    }

    chart
        .iter()
        .zip(grid.iter())
        .filter(|((_, c), _)| **c == b'0')
        .map(|(_, (_, v))| -> usize { v.values().sum() })
        .sum()
}

use crate::util::{Grid, Permutable};
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

fn find_antennas<T: Deref<Target = [u8]>>(grid: &Grid<T>) -> HashMap<u8, HashSet<(usize, usize)>> {
    let mut antennas = HashMap::new();
    for (pos, value) in grid.iter() {
        if *value != b'.' {
            let entry = antennas.entry(*value).or_insert_with(HashSet::new);
            entry.insert(pos);
        }
    }
    antennas
}

fn calc_antinodes(a: (usize, usize), b: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (x, y) = (a.0 as isize, a.1 as isize);
    let (i, j) = (b.0 as isize, b.1 as isize);
    let (dx, dy) = (i - x, j - y);
    let (x1, y1) = (x - dx, y - dy);
    let (x2, y2) = (i + dx, j + dy);
    [
        (x1 >= 0 && y1 >= 0).then_some((x1 as usize, y1 as usize)),
        (x2 >= 0 && y2 >= 0).then_some((x2 as usize, y2 as usize)),
    ]
    .into_iter()
    .flatten()
}

pub fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input);

    let antennas = find_antennas(&grid);

    let mut antinodes: HashSet<_> = HashSet::new();

    for (_, antennas) in antennas {
        for (a, b) in antennas.iter().permutations() {
            antinodes.extend(calc_antinodes(*a, *b).filter(|pos| grid.contains(*pos)));
        }
    }
    antinodes.len()
}

fn calc_all_antinodes<T>(
    grid: &Grid<T>,
    a: (usize, usize),
    b: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let (x, y) = (a.0 as isize, a.1 as isize);
    let (i, j) = (b.0 as isize, b.1 as isize);
    let (dx, dy) = (i - x, j - y);
    let mut sx = x;
    let mut sy = y;
    let mut sx2 = sx - dx;
    let mut sy2 = sy - dy;
    std::iter::from_fn(move || {
        if sx < 0 || sy < 0 {
            return None;
        }
        let result = (sx as usize, sy as usize);
        if !grid.contains(result) {
            return None;
        }
        sx += dx;
        sy += dy;
        Some(result)
    })
    .chain(std::iter::from_fn(move || {
        if sx2 < 0 || sy2 < 0 {
            return None;
        }
        let result = (sx2 as usize, sy2 as usize);
        if !grid.contains(result) {
            return None;
        }
        sx2 -= dx;
        sy2 -= dy;
        Some(result)
    }))
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input);

    let antennas = find_antennas(&grid);

    let mut antinodes: HashSet<_> = HashSet::new();

    for (_, antennas) in antennas {
        for (a, b) in antennas.iter().permutations() {
            antinodes.extend(calc_all_antinodes(&grid, *a, *b).filter(|pos| grid.contains(*pos)));
        }
    }

    antinodes.len()
}

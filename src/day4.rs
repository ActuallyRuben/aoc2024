use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    contents: Vec<T>,
}

impl<T: From<char>> FromStr for Grid<T> {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, ()> {
        let width = input.lines().next().unwrap().len();
        let contents: Vec<T> = input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| T::from(c))
            .collect();
        let height = contents.len() / width;
        Ok(Self {
            width,
            height,
            contents,
        })
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &T {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.contents[y * self.width + x]
    }
}

fn matches(grid: &Grid<char>, text: &str, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    for (i, c) in text.chars().enumerate() {
        let x = (x as isize + (i as isize) * dx) as usize;
        let y = (y as isize + (i as isize) * dy) as usize;
        if x >= grid.width || y >= grid.height {
            return false;
        }
        if grid[(x, y)] != c {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    const XMAS: &str = "XMAS";
    const SAMX: &str = "SAMX";
    (0..grid.height)
        .map(|j| {
            (0..grid.width)
                .map(|i| {
                    let mut total = 0;
                    if matches(&grid, XMAS, i, j, 1, 0) {
                        total += 1;
                    }
                    if matches(&grid, XMAS, i, j, 0, 1) {
                        total += 1;
                    }
                    if matches(&grid, XMAS, i, j, 1, 1) {
                        total += 1;
                    }
                    if matches(&grid, XMAS, i, j + 3, 1, -1) {
                        total += 1;
                    }
                    if matches(&grid, SAMX, i, j, 1, 0) {
                        total += 1;
                    }
                    if matches(&grid, SAMX, i, j, 0, 1) {
                        total += 1;
                    }
                    if matches(&grid, SAMX, i, j, 1, 1) {
                        total += 1;
                    }
                    if matches(&grid, SAMX, i, j + 3, 1, -1) {
                        total += 1;
                    }
                    total
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    const MAS: &str = "MAS";
    const SAM: &str = "SAM";
    (0..(grid.height - 2))
        .map(|j| {
            (0..(grid.width - 2))
                .map(|i| {
                    if (matches(&grid, MAS, i, j, 1, 1) || matches(&grid, SAM, i, j, 1, 1))
                        && (matches(&grid, MAS, i, j + 2, 1, -1)
                            || matches(&grid, SAM, i, j + 2, 1, -1))
                    {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

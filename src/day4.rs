use std::ops::Index;

#[derive(Debug)]
struct RefGrid<'a> {
    width: usize,
    height: usize,
    contents: &'a [u8],
}

impl<'a> RefGrid<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.len() / (width + 1);
        Self {
            width,
            height,
            contents: input.as_bytes(),
        }
    }
}

impl Index<(usize, usize)> for RefGrid<'_> {
    type Output = u8;
    fn index(&self, (x, y): (usize, usize)) -> &u8 {
        assert!(x < self.width);
        &self.contents[y * (self.width + 1) + x]
    }
}

fn matches(grid: &RefGrid, text: &[u8], x: usize, y: usize, dx: isize, dy: isize) -> bool {
    for (i, c) in text.into_iter().enumerate() {
        let x = (x as isize + (i as isize) * dx) as usize;
        let y = (y as isize + (i as isize) * dy) as usize;
        if x >= grid.width || y >= grid.height {
            return false;
        }
        if grid[(x, y)] != *c {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> usize {
    let grid = RefGrid::from_str(input);
    const XMAS: &[u8] = b"XMAS";
    const SAMX: &[u8] = b"SAMX";
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
    let grid = RefGrid::from_str(input);
    
    (0..(grid.height - 2))
        .map(|j| {
            (0..(grid.width - 2))
                .map(|i| {
                    if grid[(i + 1, j + 1)] != b'A' {
                        return 0
                    }
                    if !(grid[(i, j)] == b'M' && grid[(i + 2, j + 2)] == b'S') && !(grid[(i, j)] == b'S' && grid[(i + 2, j + 2)] == b'M') {
                        return 0;
                    }
                    if !(grid[(i + 2, j)] == b'M' && grid[(i, j + 2)] == b'S') && !(grid[(i + 2, j)] == b'S' && grid[(i, j + 2)] == b'M') {
                        return 0;
                    }
                    1
                })
                .sum::<usize>()
        })
        .sum()
}

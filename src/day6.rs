use std::ops::Add;
use crate::util::Grid;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    North, East, South, West
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = self;
        match rhs {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}

impl Add<(usize, usize)> for Direction {
    type Output = (usize, usize);

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        rhs + self
    }
}

struct Guard {
    pos: (usize, usize), direction: Direction
}

pub fn part1(input: &str) -> usize {
    let mut grid = Grid::from_str_cloned(input);
    let mut guard = None;
    for (pos, v) in grid.iter() {
        if *v == b'^' {
            guard = Some(Guard {
                pos, direction: Direction::North
            });
            break
        }
    }
    let mut guard = guard.unwrap();
    let mut visited = 0;
    
    'outer: loop {
        if grid[guard.pos] != b'X' {
            visited += 1;
            grid[guard.pos] = b'X';
        }
        'inner: loop {
            let new_pos = guard.pos + guard.direction;
            if !grid.contains(new_pos) {
                break 'outer
            }
            if grid[new_pos] != b'#' {
                guard.pos = new_pos;
                break 'inner
            }
            guard.direction = guard.direction.rotate_right();
        }
    }
    visited
}
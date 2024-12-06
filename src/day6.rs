use std::collections::HashSet;
use std::ops::Add;
use crate::util::Grid;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North, East, South, West
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'^' => Ok(Direction::North),
            b'>' => Ok(Direction::East),
            b'v' => Ok(Direction::South),
            b'<' => Ok(Direction::West),
            _ => Err(())
        }
    }
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => b'^',
            Direction::East => b'>',
            Direction::South => b'v',
            Direction::West => b'<',
        }
    }
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

#[derive(Clone, Hash, Eq, PartialEq)]
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

fn can_escape(grid: &Grid<Vec<u8>>, mut visited: HashSet<Guard>, mut guard: Guard) -> bool {
    loop {
        if visited.contains(&guard) {
            return false
        }
        visited.insert(guard.clone());
        'inner: loop {
            let new_pos = guard.pos + guard.direction;
            if !grid.contains(new_pos) {
                return true;
            }
            if grid[new_pos] != b'#' {
                guard.pos = new_pos;
                break 'inner
            }
            guard.direction = guard.direction.rotate_right();
        }
    }
}

pub fn part2(input: &str) -> usize {
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
    let mut visited = HashSet::new();
    let mut obstacles = HashSet::new();

    'outer: loop {
        'inner: loop {
            let new_pos = guard.pos + guard.direction;
            if !grid.contains(new_pos) {
                break 'outer
            }
            if grid[new_pos] == b'.' {
                grid[new_pos] = b'#';
                if !can_escape(&grid, visited.clone(), guard.clone()) {
                    obstacles.insert(new_pos);
                }
                grid[new_pos] = b'-';
            }
            if grid[new_pos] != b'#' {
                visited.insert(guard.clone());
                guard.pos = new_pos;
                break 'inner
            }
            guard.direction = guard.direction.rotate_right();
        }
    }
    for pos in &obstacles {
        grid[*pos] = b'O';
    }
    obstacles.len()
}
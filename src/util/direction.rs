use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        [Self::North, Self::East, Self::South, Self::West].into_iter()
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

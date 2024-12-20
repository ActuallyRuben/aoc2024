use crate::util::{Direction, Grid};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};

#[derive(Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Node {
    #[default]
    Empty = 0,
    Visited = 1,
    Wall = 2,
}

#[derive(Eq, PartialEq, Debug)]
struct Position {
    pos: (usize, usize),
    cost: usize,
    estimate_cost: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.estimate_cost.cmp(&other.estimate_cost).reverse()
    }
}

fn a_star(start: (usize, usize), end: (usize, usize), grid: &mut Grid<Node>) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    queue.push(Position {
        pos: start,
        cost: 0,
        estimate_cost: end.0 + end.1,
    });
    while let Some(pos) = queue.pop() {
        if pos.pos == end {
            return Some(pos.cost);
        }
        for dir in Direction::iter() {
            let new_pos = pos.pos + dir;
            if grid.contains(new_pos) && grid[new_pos] == Node::Empty {
                queue.push(Position {
                    pos: new_pos,
                    cost: pos.cost + 1,
                    estimate_cost: pos.cost
                        + 1
                        + new_pos.0.abs_diff(end.0)
                        + new_pos.1.abs_diff(end.1),
                });
                grid[new_pos] = Node::Visited;
            }
        }
    }
    None
}

pub fn part1(input: &str) -> usize {
    let input = input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    });

    let mut grid = Grid::new(71, 71);

    for point in input.take(1024) {
        grid[point] = Node::Wall;
    }

    a_star((0, 0), (70, 70), &mut grid).unwrap()
}

pub fn part2(input: &str) -> String {
    let input: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();

    for i in 1025..=input.len() {
        let mut grid = Grid::new(71, 71);

        for point in input.iter().copied().take(i) {
            grid[point] = Node::Wall;
        }
        if a_star((0, 0), (70, 70), &mut grid).is_none() {
            let (x, y) = input[i - 1];
            return format!("{x},{y}");
        }
    }
    panic!("no solution")
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Empty => write!(f, "."),
            Node::Visited => write!(f, "O"),
            Node::Wall => write!(f, "#"),
        }
    }
}

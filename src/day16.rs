use crate::util::{Direction, Grid, RefGrid};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};

#[derive(Copy, Clone)]
struct Position {
    pos: (usize, usize),
    dir: Direction,
    cost: usize,
    estimated_cost: usize,
}

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost == other.estimated_cost
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Position {}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.estimated_cost).cmp(&Reverse(other.estimated_cost))
    }
}

impl Position {
    fn estimate_cost(self, (goal_x, goal_y): (usize, usize)) -> Self {
        let (x, y) = self.pos;
        Self {
            estimated_cost: self.cost + x.abs_diff(goal_x) + y.abs_diff(goal_y),
            ..self
        }
    }

    fn do_move(
        self,
        new_pos: (usize, usize),
        new_dir: Direction,
        cost: usize,
        goal: (usize, usize),
    ) -> Self {
        Self {
            pos: new_pos,
            cost: self.cost + cost,
            dir: new_dir,
            estimated_cost: 0,
        }
        .estimate_cost(goal)
    }
}

fn dir_to_number(dir: Direction) -> usize {
    match dir {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
    }
}

fn number_to_dir(dir: usize) -> Direction {
    match dir {
        0 => Direction::North,
        1 => Direction::East,
        2 => Direction::South,
        3 => Direction::West,
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> usize {
    let grid = RefGrid::from_str(input);
    let mut visited: [Grid<bool>; 4] =
        [(); 4].map(|_| Grid::new(grid.width(), grid.height()));
    
    let mut heap = BinaryHeap::new();
    let end_pos = (grid.width() - 2, 1);
    let start_pos = Position {
        pos: (1, grid.height() - 2),
        dir: Direction::East,
        cost: 0,
        estimated_cost: 0,
    }
    .estimate_cost(end_pos);
    heap.push(start_pos);
    while let Some(pos) = heap.pop() {
        if pos.pos == end_pos {
            return pos.cost;
        }
        let new_pos = pos.pos + pos.dir;
        if grid.contains(new_pos) && grid[new_pos] != b'#' {
            let new_pos = pos.do_move(new_pos, pos.dir, 1, end_pos);
            if !visited[dir_to_number(new_pos.dir)][new_pos.pos] {
                heap.push(new_pos);
                visited[dir_to_number(new_pos.dir)][new_pos.pos] = true;
            }
        }
        let new_pos = pos.do_move(pos.pos, pos.dir.rotate_left(), 1000, end_pos);
        if !visited[dir_to_number(new_pos.dir)][new_pos.pos] {
            heap.push(new_pos);
            visited[dir_to_number(new_pos.dir)][new_pos.pos] = true;
        }
        let new_pos = pos.do_move(pos.pos, pos.dir.rotate_right(), 1000, end_pos);
        if !visited[dir_to_number(new_pos.dir)][new_pos.pos] {
            heap.push(new_pos);
            visited[dir_to_number(new_pos.dir)][new_pos.pos] = true;
        }
    }
    panic!("no solution")
}

pub fn part2(input: &str) -> usize {
    let mut grid = RefGrid::from_str_cloned(input);
    let mut visited: [Grid<Option<usize>>; 4] =
        [(); 4].map(|_| Grid::new(grid.width(), grid.height()));

    let mut heap = BinaryHeap::new();
    let end_pos = (grid.width() - 2, 1);
    let start_pos = Position {
        pos: (1, grid.height() - 2),
        dir: Direction::East,
        cost: 0,
        estimated_cost: 0,
    }
    .estimate_cost(end_pos);
    heap.push(start_pos);
    visited[dir_to_number(start_pos.dir)][start_pos.pos] = Some(0);
    while let Some(pos) = heap.pop() {
        let new_pos = pos.pos + pos.dir;
        if grid.contains(new_pos) && grid[new_pos] != b'#' {
            let new_pos = pos.do_move(new_pos, pos.dir, 1, end_pos);
            let cell = &mut visited[dir_to_number(new_pos.dir)][new_pos.pos];
            if cell.is_none_or(|prev_cost| prev_cost > new_pos.cost) {
                heap.push(new_pos);
                *cell = Some(new_pos.cost);
            }
        }
        let new_pos = pos.do_move(pos.pos, pos.dir.rotate_left(), 1000, end_pos);
        let cell = &mut visited[dir_to_number(new_pos.dir)][new_pos.pos];
        if cell.is_none_or(|prev_cost| prev_cost > new_pos.cost) {
            heap.push(new_pos);
            *cell = Some(new_pos.cost);
        }
        let new_pos = pos.do_move(pos.pos, pos.dir.rotate_right(), 1000, end_pos);
        let cell = &mut visited[dir_to_number(new_pos.dir)][new_pos.pos];
        if cell.is_none_or(|prev_cost| prev_cost > new_pos.cost) {
            heap.push(new_pos);
            *cell = Some(new_pos.cost);
        }
    }
    let exit_dir = visited
        .iter()
        .enumerate()
        .filter(|(_, grid)| grid[end_pos].is_some())
        .map(|(i, grid)| (end_pos, i, grid[end_pos].unwrap()))
        .min_by_key(|(_, _, score)| *score)
        .unwrap();
    let mut queue: VecDeque<((usize, usize), usize, usize)> = VecDeque::from([exit_dir]);
    let mut count = 0;
    while let Some((pos, dir, cost)) = queue.pop_front() {
        if grid[pos] != b'O' {
            grid[pos] = b'O';
            count += 1;
        }
        let rotate_left = (dir - 1) % 4;
        if let Some(left_cost) = visited[rotate_left][pos] {
            if left_cost == cost - 1000 {
                queue.push_back((pos, rotate_left, left_cost))
            }
        }
        let rotate_right = (dir + 1) % 4;
        if let Some(right_cost) = visited[rotate_right][pos] {
            if right_cost == cost - 1000 {
                queue.push_back((pos, rotate_right, right_cost))
            }
        }
        let new_pos = pos + -number_to_dir(dir);
        if let Some(forward_cost) = visited[dir]
            .contains(new_pos)
            .then(|| visited[dir][new_pos])
            .flatten()
        {
            if forward_cost == cost - 1 {
                queue.push_back((new_pos, dir, forward_cost));
            }
        }
    }
    count
}

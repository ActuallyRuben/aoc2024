use crate::util::{Direction, Grid, RefGrid};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Node {
    pos: (usize, usize),
    cost: usize,
    estimate: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.estimate.cmp(&other.estimate).reverse()
    }
}

fn manhattan((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn find_shortcuts(costs: &Grid<usize>, min_timesave: usize) -> usize {
    let mut count = 0;
    for (pos, cost) in costs.iter().filter(|(_, cost)| **cost == usize::MAX) {
        for entry_dir in Direction::iter()
            .filter(|dir| costs.contains(pos + *dir) && costs[pos + *dir] != usize::MAX)
        {
            for exit_dir in Direction::iter().filter(|exit_dir| {
                *exit_dir != entry_dir
                    && costs.contains(pos + *exit_dir)
                    && costs[pos + *exit_dir] != usize::MAX
            }) {
                if costs[pos + exit_dir] + 2 < costs[pos + entry_dir] {
                    let time_save = costs[pos + entry_dir] - costs[pos + exit_dir] - 2;
                    if time_save >= min_timesave {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn find_long_shortcuts(costs: &Grid<usize>, distance: usize, min_timesave: usize) -> usize {
    let mut count = 0;
    for ((x, y), cost) in costs.iter().filter(|(_, cost)| **cost != usize::MAX) {
        for dx in -(distance as isize)..=(distance as isize) {
            let y_distance = distance as isize - dx.abs();
            for dy in -y_distance..=y_distance {
                let entry_pos = (x, y);
                let exit_pos = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                if !costs.contains(exit_pos) || costs[exit_pos] == usize::MAX {
                    continue;
                }
                let cur_distance = (dx.abs() + dy.abs()) as usize;
                if costs[exit_pos] + cur_distance < costs[entry_pos] {
                    let time_save = costs[entry_pos] - costs[exit_pos] - cur_distance;
                    if time_save >= min_timesave {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn a_star(
    grid: &RefGrid<&[u8]>,
    costs: &mut Grid<usize>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) {
    let mut queue = BinaryHeap::new();
    queue.push(Node {
        pos: end_pos,
        cost: 0,
        estimate: manhattan(start_pos, end_pos),
    });
    costs[end_pos] = 0;
    while let Some(node) = queue.pop() {
        for dir in Direction::iter() {
            let new_pos = node.pos + dir;
            let new_cost = node.cost + 1;
            if !grid.contains(new_pos) || grid[new_pos] == b'#' || costs[new_pos] <= new_cost {
                continue;
            }
            costs[new_pos] = new_cost;
            queue.push(Node {
                pos: new_pos,
                cost: new_cost,
                estimate: new_cost + manhattan(new_pos, start_pos),
            })
        }
    }
}

pub fn part1(input: &str) -> usize {
    let grid = RefGrid::from_str(input);
    let mut costs = Grid::new(grid.width(), grid.height());
    let mut start_pos = None;
    let mut end_pos = None;
    for (pos, point) in costs.iter_mut() {
        *point = usize::MAX;
        match grid[pos] {
            b'S' => start_pos = Some(pos),
            b'E' => end_pos = Some(pos),
            _ => {}
        }
    }
    let start_pos = start_pos.unwrap();
    let end_pos = end_pos.unwrap();

    a_star(&grid, &mut costs, start_pos, end_pos);

    find_shortcuts(&costs, 100)
}

pub fn part2(input: &str) -> usize {
    let grid = RefGrid::from_str(input);
    let mut costs = Grid::new(grid.width(), grid.height());
    let mut start_pos = None;
    let mut end_pos = None;
    for (pos, point) in costs.iter_mut() {
        *point = usize::MAX;
        match grid[pos] {
            b'S' => start_pos = Some(pos),
            b'E' => end_pos = Some(pos),
            _ => {}
        }
    }
    let start_pos = start_pos.unwrap();
    let end_pos = end_pos.unwrap();

    a_star(&grid, &mut costs, start_pos, end_pos);

    find_long_shortcuts(&costs, 20, 100)
}

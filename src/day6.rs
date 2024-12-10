use crate::util::{Direction, RefGrid};
use std::collections::HashSet;
use std::sync::Mutex;
use std::thread;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Guard {
    pos: (usize, usize),
    direction: Direction,
}

pub fn part1(input: &str) -> usize {
    let mut grid = RefGrid::from_str_cloned(input);
    let mut guard = None;
    for (pos, v) in grid.iter() {
        if *v == b'^' {
            guard = Some(Guard {
                pos,
                direction: Direction::North,
            });
            break;
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
                break 'outer;
            }
            if grid[new_pos] != b'#' {
                guard.pos = new_pos;
                break 'inner;
            }
            guard.direction = guard.direction.rotate_right();
        }
    }
    visited
}

fn can_escape(
    grid: &RefGrid<&[u8]>,
    extra_obstacle: (usize, usize),
    mut visited: HashSet<Guard>,
    mut guard: Guard,
) -> bool {
    loop {
        if visited.contains(&guard) {
            return false;
        }
        visited.insert(guard.clone());
        'inner: loop {
            let new_pos = guard.pos + guard.direction;
            if !grid.contains(new_pos) {
                return true;
            }
            if new_pos != extra_obstacle && grid[new_pos] != b'#' {
                guard.pos = new_pos;
                break 'inner;
            }
            guard.direction = guard.direction.rotate_right();
        }
    }
}

pub fn part2(input: &str) -> usize {
    let grid = RefGrid::from_str(input);
    let mut guard = None;
    for (pos, v) in grid.iter() {
        if *v == b'^' {
            guard = Some(Guard {
                pos,
                direction: Direction::North,
            });
            break;
        }
    }

    let mut guard = guard.unwrap();
    let mut checked = HashSet::new();
    let mut visited = HashSet::new();
    let obstacles = Mutex::new(HashSet::new());
    thread::scope(|s| 'outer: loop {
        'inner: loop {
            let new_pos = guard.pos + guard.direction;
            if !grid.contains(new_pos) {
                break 'outer;
            }
            if grid[new_pos] == b'.' && !checked.contains(&new_pos) {
                let inner_guard = guard.clone();
                let inner_visited = visited.clone();
                let inner_obstacles = &obstacles;
                let inner_grid = &grid;
                s.spawn(move || {
                    if !can_escape(inner_grid, new_pos, inner_visited, inner_guard) {
                        inner_obstacles.lock().unwrap().insert(new_pos);
                    }
                });
                checked.insert(new_pos);
            }
            if grid[new_pos] != b'#' {
                visited.insert(guard.clone());
                guard.pos = new_pos;
                break 'inner;
            }
            guard.direction = guard.direction.rotate_right();
        }
    });

    obstacles.into_inner().unwrap().len()
}

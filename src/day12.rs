use crate::util::{Direction, RefGrid};
use std::collections::VecDeque;

fn get_plot_perimeter_area(
    location: (usize, usize),
    grid: &mut RefGrid<Vec<u8>>,
) -> (usize, usize) {
    let plant_type = grid[location];
    grid[location] |= 0x80;
    debug_assert_eq!(
        plant_type & 0x80,
        0,
        "{location:?} has already been visited"
    );
    let mut queue = VecDeque::from([location]);
    let mut area = 0;
    let mut perimeter = 0;

    while let Some(location) = queue.pop_front() {
        area += 1;

        for dir in Direction::iter() {
            let new_pos = location + dir;
            if !grid.contains(new_pos) {
                perimeter += 1;
                continue;
            }
            if grid[new_pos] & 0x7f == plant_type {
                if grid[new_pos] & 0x80 == 0x00 {
                    queue.push_back(new_pos);
                    grid[new_pos] |= 0x80;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (perimeter, area)
}

pub fn part1(input: &str) -> usize {
    let mut grid = RefGrid::from_str_cloned(input);
    let mut pricing = 0;
    for j in 0..grid.height() {
        for i in 0..grid.width() {
            if grid[(i, j)] & 0x80 == 0 {
                let (perimeter, area) = get_plot_perimeter_area((i, j), &mut grid);
                pricing += perimeter * area;
            }
        }
    }
    pricing
}

fn get_plot_sides_area(location: (usize, usize), grid: &mut RefGrid<Vec<u8>>) -> (usize, usize) {
    let plant_type = grid[location] & 0x7f;
    grid[location] |= 0x80;
    debug_assert_eq!(
        plant_type & 0x80,
        0,
        "{location:?} has already been visited"
    );
    let mut queue = VecDeque::from([location]);
    let mut sides = 0;
    let mut area = 0;

    while let Some(location) = queue.pop_front() {
        area += 1;
        sides += count_corners(location, grid);

        for dir in Direction::iter() {
            let new_pos = location + dir;
            if !grid.contains(new_pos) {
                continue;
            }
            if grid[new_pos] & 0x7f == plant_type {
                if grid[new_pos] & 0x80 == 0x00 {
                    queue.push_back(new_pos);
                    grid[new_pos] |= 0x80;
                }
            }
        }
    }

    (sides, area)
}

fn count_corners(location: (usize, usize), grid: &mut RefGrid<Vec<u8>>) -> usize {
    let plant_type = grid[location] & 0x7f;
    let mut corners = 0;
    for dir in Direction::iter() {
        let pos1 = location + dir;
        let pos2 = location + dir.rotate_right();
        let pos3 = location + dir + dir.rotate_right();
        if (!grid.contains(pos1) || grid[pos1] & 0x7f != plant_type)
            && (!grid.contains(pos2) || grid[pos2] & 0x7f != plant_type)
        {
            corners += 1;
        } else if (grid.contains(pos1) && grid[pos1] & 0x7f == plant_type)
            && (grid.contains(pos2) && grid[pos2] & 0x7f == plant_type)
            && (!grid.contains(pos3) || grid[pos3] & 0x7f != plant_type)
        {
            corners += 1;
        }
    }
    corners
}

pub fn part2(input: &str) -> usize {
    let mut grid = RefGrid::from_str_cloned(input);
    let mut pricing = 0;
    for j in 0..grid.height() {
        for i in 0..grid.width() {
            if grid[(i, j)] & 0x80 == 0 {
                let (sides, area) = get_plot_sides_area((i, j), &mut grid);
                pricing += sides * area;
            }
        }
    }
    pricing
}

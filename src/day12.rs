use std::collections::VecDeque;
use crate::util::{Direction, RefGrid};

fn get_plot_perimeter_area(location: (usize, usize), grid: &mut RefGrid<Vec<u8>>) -> (usize, usize) {
    let plant_type = grid[location];
    grid[location] |= 0x80;
    debug_assert_eq!(plant_type & 0x80, 0, "{location:?} has already been visited");
    let mut queue = VecDeque::from([location]);
    let mut area = 0;
    let mut perimeter = 0;
    
    while let Some(location) = queue.pop_front() {
        area += 1;

        for dir in Direction::iter() {
            let new_pos = location + dir;
            if !grid.contains(new_pos) {
                perimeter += 1;
                continue
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
use crate::util::{Direction, RefGrid};
use std::convert::identity;
use std::ops::{Deref, DerefMut};

fn calc_score(grid: &RefGrid<impl Deref<Target = [u8]>>, target: u8) -> usize {
    grid.iter()
        .filter(|(_, c)| **c == target)
        .map(|((x, y), _)| 100 * y + x)
        .sum()
}

fn try_move(
    object: (usize, usize),
    direction: Direction,
    grid: &mut RefGrid<impl DerefMut<Target = [u8]>>,
) -> Result<(usize, usize), (usize, usize)> {
    let new_pos = object + direction;
    let space_available = match grid[new_pos] {
        b'.' => true,
        b'#' => false,
        b'O' => try_move(new_pos, direction, grid).is_ok(),
        _ => unreachable!(),
    };
    if space_available {
        grid[new_pos] = grid[object];
        grid[object] = b'.';
        Ok(new_pos)
    } else {
        Err(object)
    }
}

pub fn part1(input: &str) -> usize {
    let (grid, movements) = input.split_once("\n\n").unwrap();
    let mut grid = RefGrid::from_str_cloned(grid);

    let mut robot = grid.iter().find(|(_, c)| **c == b'@').unwrap().0;

    for dir in movements
        .chars()
        .filter_map(|c| Direction::try_from(c).ok())
    {
        robot = try_move(robot, dir, &mut grid).map_or_else(identity, identity);
    }

    calc_score(&grid, b'O')
}

fn can_move(
    object: (usize, usize),
    direction: Direction,
    grid: &RefGrid<impl Deref<Target = [u8]>>,
) -> bool {
    let (x, y) = object + direction;
    match grid[(x, y)] {
        b'.' => true,
        b'#' => false,
        b'[' => match direction {
            Direction::North | Direction::South => {
                can_move((x, y), direction, grid) && can_move((x + 1, y), direction, grid)
            }
            Direction::East => can_move((x + 1, y), direction, grid),
            Direction::West => panic!("invalid box"),
        },
        b']' => match direction {
            Direction::North | Direction::South => {
                can_move((x - 1, y), direction, grid) && can_move((x, y), direction, grid)
            }
            Direction::East => panic!("invalid box"),
            Direction::West => can_move((x - 1, y), direction, grid),
        },
        _ => unreachable!(),
    }
}

fn do_move(
    object: (usize, usize),
    direction: Direction,
    grid: &mut RefGrid<impl DerefMut<Target = [u8]>>,
) {
    let (x, y) = object + direction;
    match grid[(x, y)] {
        b'.' => {}
        b'[' => match direction {
            Direction::North | Direction::South => {
                do_move((x, y), direction, grid);
                do_move((x + 1, y), direction, grid);
            }
            Direction::East | Direction::West => do_move((x, y), direction, grid),
        },
        b']' => match direction {
            Direction::North | Direction::South => {
                do_move((x - 1, y), direction, grid);
                do_move((x, y), direction, grid);
            }
            Direction::East | Direction::West => do_move((x, y), direction, grid),
        },
        _ => unreachable!(),
    };
    grid[(x, y)] = grid[object];
    grid[object] = b'.';
}

pub fn part2(input: &str) -> usize {
    let (grid, movements) = input.split_once("\n\n").unwrap();

    let grid: String = grid
        .chars()
        .flat_map(|c| match c {
            '#' => "##".chars(),
            'O' => "[]".chars(),
            '.' => "..".chars(),
            '@' => "@.".chars(),
            '\n' => "\n".chars(),
            _ => unreachable!("invalid char '{}'", c),
        })
        .collect();

    let mut grid = RefGrid::from_string(grid);

    let mut robot = grid.iter().find(|(_, c)| **c == b'@').unwrap().0;

    for dir in movements
        .chars()
        .filter_map(|c| Direction::try_from(c).ok())
    {
        if can_move(robot, dir, &grid) {
            do_move(robot, dir, &mut grid);
            robot += dir;
        }
    }

    calc_score(&grid, b'[')
}

use crate::util::Grid;
use std::collections::HashSet;
use std::str::FromStr;
use rayon::prelude::*;

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

struct Robot {
    p: (isize, isize),
    v: (isize, isize),
}

impl Robot {
    fn simulate(&mut self, steps: usize) {
        self.p.0 += steps as isize * self.v.0;
        self.p.0 = self.p.0.rem_euclid(WIDTH);
        self.p.1 += steps as isize * self.v.1;
        self.p.1 = self.p.1.rem_euclid(HEIGHT);
    }

    fn quadrant(&self) -> Option<usize> {
        let (x, y) = self.p;
        let qx = if x < (WIDTH / 2) {
            0
        } else if x > (WIDTH / 2) {
            1
        } else {
            return None;
        };
        let qy = if y < (HEIGHT / 2) {
            0
        } else if y > (HEIGHT / 2) {
            2
        } else {
            return None;
        };
        Some(qx + qy)
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.splitn(2, ' ');
        let mut p = splitted.next().ok_or(())?[2..].splitn(2, ',');
        let px: isize = p.next().ok_or(())?.parse().map_err(|_| ())?;
        let py: isize = p.next().ok_or(())?.parse().map_err(|_| ())?;
        let mut v = splitted.next().ok_or(())?[2..].splitn(2, ',');
        let vx: isize = v.next().ok_or(())?.parse().map_err(|_| ())?;
        let vy: isize = v.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Robot {
            p: (px, py),
            v: (vx, vy),
        })
    }
}

#[allow(unused)]
fn print_robots(robots: &[Robot]) {
    let mut grid: Grid<char> = Grid::new(WIDTH as usize, HEIGHT as usize);
    for (_, v) in grid.iter_mut() {
        *v = '0';
    }
    for robot in robots {
        let (x, y) = robot.p;
        grid[(x as usize, y as usize)] = (grid[(x as usize, y as usize)] as u8 + 1) as char;
    }
    for (_, v) in grid.iter_mut() {
        if *v == '0' {
            *v = ' ';
        }
    }
    println!("{grid}");
}

pub fn part1(input: &str) -> usize {
    let robots: Vec<Robot> = input
        .lines()
        .map(|line| line.parse::<Robot>().unwrap())
        .map(|mut robot| {
            robot.simulate(100);
            robot
        })
        .collect();
    let mut quadrants = [0; 4];
    for quadrant in robots.iter().filter_map(|r| r.quadrant()) {
        quadrants[quadrant] += 1;
    }
    quadrants.into_iter().product()
}

pub fn part2(input: &str) -> usize {
    let mut robots: Vec<(Robot, usize)> = input.lines().map(|line| (line.parse().unwrap(), 0)).collect();
    let mut seconds = 0;
    let mut position_set = HashSet::new();
    'outer: loop {
        seconds += 1;
        
        for (robot, last_sim) in &mut robots {
            robot.simulate(seconds - *last_sim);
            *last_sim = seconds;
            if !position_set.insert(robot.p) {
                position_set.clear();
                continue 'outer;
            }
        }
        break;
    }
    seconds
}

use crate::util::Grid;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;

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
        let qx = match x.cmp(&(WIDTH / 2)) {
            Ordering::Less => 0,
            Ordering::Greater => 1,
            Ordering::Equal => return None,
        };
        let qy = match y.cmp(&(HEIGHT / 2)) {
            Ordering::Less => 0,
            Ordering::Greater => 2,
            Ordering::Equal => return None,
        };
        Some(qx + qy)
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').ok_or(())?;
        let (px, py) = p[2..].split_once(',').ok_or(())?;
        let px: isize = px.parse().map_err(|_| ())?;
        let py: isize = py.parse().map_err(|_| ())?;
        let (vx, vy) = v[2..].split_once(',').ok_or(())?;
        let vx: isize = vx.parse().map_err(|_| ())?;
        let vy: isize = vy.parse().map_err(|_| ())?;
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
    let mut robots: Vec<(Robot, usize)> = input
        .lines()
        .map(|line| (line.parse().unwrap(), 0))
        .collect();
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

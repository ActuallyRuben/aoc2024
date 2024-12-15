#[derive(Debug)]
struct Machine {
    axy: (isize, isize),
    bxy: (isize, isize),
    prize: (isize, isize),
}

fn parse_button(input: &str) -> (isize, isize) {
    let (x, y) = input.split_once(',').unwrap();
    let x = x[12..].parse().unwrap();
    let y = y[3..].parse().unwrap();
    (x, y)
}

fn parse_machine(input: &str) -> Machine {
    let mut input = input.lines();
    let axy = parse_button(input.next().unwrap());
    let bxy = parse_button(input.next().unwrap());
    let (prize_x, prize_y) = input.next().unwrap().split_once(',').unwrap();
    let prize_x = prize_x[9..].parse().unwrap();
    let prize_y = prize_y[3..].parse().unwrap();
    Machine {
        axy,
        bxy,
        prize: (prize_x, prize_y),
    }
}

fn solve_machine(machine: Machine) -> Option<isize> {
    let a = machine.axy.0 as f64;
    let c = machine.axy.1 as f64;
    let b = machine.bxy.0 as f64;
    let d = machine.bxy.1 as f64;
    let det = a * d - b * c;
    let e = machine.prize.0 as f64;
    let f = machine.prize.1 as f64;

    let a_inv = d / det;
    let b_inv = -b / det;
    let c_inv = -c / det;
    let d_inv = a / det;

    let x = a_inv * e + b_inv * f;
    let y = c_inv * e + d_inv * f;

    if (x - x.round()).abs() < 0.0001 && (y - y.round()).abs() < 0.0001 {
        let tokens = 3.0 * x + y;
        Some(tokens.round() as isize)
    } else {
        None
    }
}

pub fn part1(input: &str) -> isize {
    input
        .split("\n\n")
        .map(parse_machine)
        .filter_map(solve_machine)
        .sum()
}

pub fn part2(input: &str) -> isize {
    input
        .split("\n\n")
        .map(parse_machine)
        .map(|mut m| {
            m.prize.0 += 10000000000000;
            m.prize.1 += 10000000000000;
            m
        })
        .filter_map(solve_machine)
        .sum()
}

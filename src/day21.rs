use std::cmp::Ordering;
use std::collections::HashMap;

fn make_movements(
    (x, y): (isize, isize),
    (next_x, next_y): (isize, isize),
    banned_pos: (isize, isize),
) -> Vec<String> {
    let x_dir = match next_x.cmp(&x) {
        Ordering::Less => '<',
        Ordering::Equal => '+',
        Ordering::Greater => '>',
    };
    let x_amt = next_x.abs_diff(x);
    let y_dir = match next_y.cmp(&y) {
        Ordering::Less => '^',
        Ordering::Equal => '-',
        Ordering::Greater => 'v',
    };
    let mut options = vec![];
    let y_amt = next_y.abs_diff(y);
    if (next_x, y) != banned_pos {
        let mut seq = String::new();
        for _ in 0..x_amt {
            seq.push(x_dir)
        }
        for _ in 0..y_amt {
            seq.push(y_dir)
        }
        seq.push('A');
        options.push(seq);
    }
    if (x, next_y) != banned_pos {
        let mut seq = String::new();
        for _ in 0..y_amt {
            seq.push(y_dir)
        }
        for _ in 0..x_amt {
            seq.push(x_dir)
        }
        seq.push('A');
        options.push(seq);
    }
    options
}

fn input_num_code(code: &str, robot_count: usize, cache: &mut [HashMap<((isize, isize), (isize, isize)), usize>]) -> usize {
    let mut pos = (2, 3);
    let positions = [
        (1, 3),
        (0, 2),
        (1, 2),
        (2, 2),
        (0, 1),
        (1, 1),
        (2, 1),
        (0, 0),
        (1, 0),
        (2, 0),
    ];
    let mut length = 0;
    for c in code.chars() {
        let next_pos = match c {
            '0'..='9' => positions[(c as u8 - b'0') as usize],
            'A' => (2, 3),
            _ => panic!("invalid char {c}"),
        };
        if let Some(v) = cache[0].get(&(pos, next_pos)) {
            length += *v;
        } else {
            let options = make_movements(pos, next_pos, (0, 3));
            let best = options.into_iter().map(|x| input_dir_code(x, robot_count, &mut cache[1..])).min().unwrap();
            cache[0].insert((pos, next_pos), best);
            length += best;
        }
        pos = next_pos;
    }
    length
}

fn input_dir_code(code: String, depth: usize, cache: &mut [HashMap<((isize, isize), (isize, isize)), usize>]) -> usize {
    if depth == 0 {
        return code.len();
    }
    let mut pos = (2, 0);
    let mut length = 0;
    for c in code.chars() {
        let next_pos = match c {
            '^' => (1, 0),
            'A' => (2, 0),
            '<' => (0, 1),
            'v' => (1, 1),
            '>' => (2, 1),
            _ => panic!("invalid char {c}"),
        };
        if let Some(v) = cache[0].get(&(pos, next_pos)) {
            length += *v;
        } else {
            let options = make_movements(pos, next_pos, (0, 0));
            let best = options.into_iter().map(|x| input_dir_code(x, depth - 1, &mut cache[1..])).min().unwrap();
            cache[0].insert((pos, next_pos), best);
            length += best;
        }
        pos = next_pos;
    }
    length
}

pub fn part1(input: &str) -> usize {
    let mut cache = [(); 3].map(|_| HashMap::new());
    input.lines().map(|code| {
        let value: usize = code[0..3].parse().unwrap();
        let input_length = input_num_code(code, 2, &mut cache);
        input_length * value
    }).sum()
}

pub fn part2(input: &str) -> usize {
    let mut cache = [(); 26].map(|_| HashMap::new());
    input.lines().map(|code| {
        let value: usize = code[0..3].parse().unwrap();
        let input_length = input_num_code(code, 25, &mut cache);
        input_length * value
    }).sum()
}

use std::collections::{HashMap, VecDeque};

fn next_secret(mut secret: u32) -> u32 {
    secret ^= secret << 6;
    secret &= 0xFFFFFF;
    secret ^= secret >> 5;
    secret &= 0xFFFFFF;
    secret ^= secret << 11;
    secret &= 0xFFFFFF;
    secret
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|mut x| {
            for _ in 0..2000 {
                x = next_secret(x)
            }
            x as usize
        })
        .sum()
}

fn price(secret: u32) -> i32 {
    (secret % 10) as i32
}

fn make_changelist(mut secret: u32) -> HashMap<[i32; 4], i32> {
    let mut window = VecDeque::with_capacity(4);
    let mut output = HashMap::new();

    for _ in 0..2000 {
        let new_secret = next_secret(secret);
        window.push_back(price(new_secret) - price(secret));
        if window.len() == 4 {
            let key = [window[0], window[1], window[2], window[3]];
            let value = price(new_secret);
            output.entry(key).or_insert(value);
            window.pop_front();
        }
        secret = new_secret;
    }

    output
}

pub fn part2(input: &str) -> i32 {
    let changes = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|secret| make_changelist(secret));
    let mut total = HashMap::new();

    for seller in changes {
        for (k, v) in seller {
            let entry = total.entry(k).or_insert(0);
            *entry += v;
        }
    }
    *total.values().max().unwrap()
}

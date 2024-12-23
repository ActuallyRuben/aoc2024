use rayon::prelude::*;
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
    let mut inputs: Vec<_> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    for _ in 0..2000 {
        for secret in &mut inputs {
            *secret = next_secret(*secret);
        }
    }
    inputs.iter().copied().map(|x| x as usize).sum()
}

fn price(secret: u32) -> i32 {
    (secret % 10) as i32
}

fn make_changelist_multi(secret: &mut [u32]) -> [HashMap<[i32; 4], i32>; 32] {
    let mut window = [(); 32].map(|_| VecDeque::with_capacity(4));
    let mut output = [(); 32].map(|_| HashMap::new());

    for _ in 0..2000 {
        for i in 0..32 {
            let secret = &mut secret[i];
            let window = &mut window[i];
            let output = &mut output[i];

            let new_secret = next_secret(*secret);
            window.push_back(price(new_secret) - price(*secret));
            if window.len() == 4 {
                let key = [window[0], window[1], window[2], window[3]];
                let value = price(new_secret);
                output.entry(key).or_insert(value);
                window.pop_front();
            }
            *secret = new_secret;
        }
    }

    output
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
    let mut secrets: Vec<u32> = input.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    let mut chunk_iter = secrets.par_chunks_exact_mut(32);
    let remainder = chunk_iter.take_remainder();
    let changes = chunk_iter
        .flat_map_iter(|x| make_changelist_multi(x))
        .chain(remainder.iter().map(|x| make_changelist(*x)).par_bridge())
        .collect_vec_list();

    let mut total = HashMap::new();

    for seller in changes.into_iter().flatten() {
        for (k, v) in seller {
            let entry = total.entry(k).or_insert(0);
            *entry += v;
        }
    }

    *total.values().max().unwrap()
}

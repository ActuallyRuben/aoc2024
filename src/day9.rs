use std::cmp::Ordering;

pub fn part1(input: &str) -> usize {
    let mut input: Vec<u8> = if input.len() % 2 == 1 {
        input.as_bytes().into()
    } else {
        input[0..input.len() - 1].as_bytes().into()
    };
    let mut result = 0;
    let mut input: &mut [u8] = &mut input;
    let mut start_file_index = 0;
    let mut end_file_index = (input.len()) / 2;
    let mut position = 0;
    loop {
        for _ in 0..(input[0] - b'0') {
            result += start_file_index * position;
            position += 1;
        }
        start_file_index += 1;
        input = &mut input[1..];
        
        if input.is_empty() {
            break;
        }
        
        'inner: loop {
            match input[0].cmp(&input[input.len() - 1]) {
                Ordering::Less => {
                    for _ in 0..(input[0] - b'0') {
                        result += end_file_index * position;
                        position += 1;
                    }
                    *input.last_mut().unwrap() -= input[0] - b'0'; 
                    input = &mut input[1..];
                    break 'inner;
                }
                Ordering::Equal => {
                    for _ in 0..(input[0] - b'0') {
                        result += end_file_index * position;
                        position += 1;
                    }
                    let new_end = input.len() - 2;
                    end_file_index -= 1;
                    input = &mut input[1..new_end];
                    break 'inner;
                }
                Ordering::Greater => {
                    for _ in 0..(input.last().unwrap() - b'0') {
                        result += end_file_index * position;
                        position += 1;
                    }
                    input[0] -= *input.last().unwrap() - b'0';
                    let new_end = input.len() - 2;
                    end_file_index -= 1;
                    input = &mut input[..new_end];
                }
            };
        }
    }
    result
}

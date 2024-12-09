use std::cmp::Ordering;
use std::collections::LinkedList;
use std::ops::ControlFlow;

fn parse_filesystem<B: FromIterator<Space>>(input: &str) -> B {
    input
        .trim()
        .as_bytes()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, chars)| {
            [
                Some(Space::File {
                    moved: false,
                    id,
                    space: chars[0] - b'0',
                }),
                chars.get(1).map(|c| Space::Free { space: *c - b'0' }),
            ]
        })
        .flatten()
        .collect()
}

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

pub enum Space {
    File { moved: bool, id: usize, space: u8 },
    Free { space: u8 },
}

pub fn part2(input: &str) -> usize {
    let mut filesystem: LinkedList<Space> = parse_filesystem(input);

    loop {
        let Some((file_id, file_space)) = filesystem.iter_mut().rev().find_map(|x| match x {
            Space::File { moved, id, space } if !*moved => {
                let id = *id;
                let space = *space;
                *x = Space::Free { space };
                Some((id, space))
            }
            _ => None,
        }) else {
            break;
        };

        map_linkedlist(&mut filesystem, |part| match part.front_mut().unwrap() {
            Space::File { .. } => ControlFlow::Continue(()),
            Space::Free { space } => match (*space).cmp(&file_space) {
                Ordering::Greater => {
                    *space -= file_space;
                    part.push_front(Space::File {
                        moved: true,
                        id: file_id,
                        space: file_space,
                    });
                    ControlFlow::Break(())
                }
                Ordering::Equal => {
                    *part.front_mut().unwrap() = Space::File {
                        moved: true,
                        id: file_id,
                        space: file_space,
                    };
                    ControlFlow::Break(())
                }
                Ordering::Less => ControlFlow::Continue(()),
            },
        });
    }

    let mut position = 0;
    let mut result = 0;

    for file in filesystem {
        match file {
            Space::File { id, space, .. } => {
                for _ in 0..space {
                    result += position * id;
                    position += 1;
                }
            }
            Space::Free { space } => {
                for _ in 0..space {}
                position += space as usize;
            }
        }
    }

    result
}

fn map_linkedlist<T>(
    list: &mut LinkedList<T>,
    mut f: impl FnMut(&mut LinkedList<T>) -> ControlFlow<()>,
) {
    let mut before = LinkedList::new();
    while !list.is_empty() {
        let mut after = list.split_off(1);
        let result = f(list);
        before.append(list);
        if result.is_continue() {
            *list = after;
        } else {
            before.append(&mut after);
        }
    }
    *list = before;
}

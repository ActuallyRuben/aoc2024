pub fn part1(input: &str) -> usize {
    let mut input: Vec<[u8]> = if input.len() % 2 == 1 {
        input.as_bytes().into()
    } else {
        input[0..input.len() - 1].as_bytes().into()
    };
    let mut result = 0;
    let mut input: &mut [u8] = &mut input;
    let mut i = 0;
    while i < input.len() - 1 {
        
    }
    0
}

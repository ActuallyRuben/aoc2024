fn run_program(mut reg_a: usize, mut reg_b: usize, mut reg_c: usize, program: &[usize]) -> Vec<usize> {
    let mut ip = 0;
    let mut output = Vec::new();
    loop {
        if ip >= program.len() {
            break
        }
        let instr = program[ip];
        let op = program[ip + 1];
        ip += 2;
        let combo = match op {
            0..=3 => op,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => 0,
        };
        match instr {
            0 => reg_a = reg_a >> combo,
            1 => reg_b ^= op,
            2 => reg_b = combo % 8,
            3 => if reg_a != 0 {
                ip = op;
            }
            4 => reg_b ^= reg_c,
            5 => output.push(combo % 8),
            6 => reg_b = reg_a >> combo,
            7 => reg_c = reg_a >> combo,
            _ => panic!(),
        }
    }
    output
}

pub fn part1(input: &str) -> String {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut registers = registers.lines().map(|reg| reg[12..].parse::<usize>().unwrap());
    let mut reg_a = registers.next().unwrap();
    let mut reg_b = registers.next().unwrap();
    let mut reg_c = registers.next().unwrap();

    let program: Vec<usize> = program[9..].trim().split(',').map(|v| v.parse::<usize>().unwrap()).collect();

    run_program(reg_a, reg_b, reg_c, &program).into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
}

pub fn part2(input: &str) -> usize {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut registers = registers.lines().map(|reg| reg[12..].parse::<usize>().unwrap());
    let mut reg_a = registers.next().unwrap();
    let mut reg_b = registers.next().unwrap();
    let mut reg_c = registers.next().unwrap();

    let program: Vec<usize> = program[9..].trim().split(',').map(|v| v.parse::<usize>().unwrap()).collect();

    for reg_a in 0.. {
        if program == run_program(reg_a, reg_b, reg_c, &program) {
            return reg_a;
        }
    }
    panic!()
}

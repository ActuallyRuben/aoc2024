struct MachineState<'a> {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    program: &'a [usize],
}

impl<'a> MachineState<'a> {
    fn new(a: usize, b: usize, c: usize, program: &'a [usize]) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            program,
        }
    }
}

impl Iterator for MachineState<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ip >= self.program.len() {
                return None;
            }
            let instr = self.program[self.ip];
            let op = self.program[self.ip + 1];
            self.ip += 2;
            let combo = match op {
                0..=3 => op,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => 0,
            };
            match instr {
                0 => self.a >>= combo,
                1 => self.b ^= op,
                2 => self.b = combo % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = op;
                    }
                }
                4 => self.b ^= self.c,
                5 => return Some(combo % 8),
                6 => self.b = self.a >> combo,
                7 => self.c = self.a >> combo,
                _ => panic!(),
            }
        }
    }
}

fn run_program(mut a: usize, mut b: usize, mut c: usize, program: &[usize]) -> Vec<usize> {
    let mut ip = 0;
    let mut output = Vec::new();
    loop {
        if ip >= program.len() {
            break;
        }
        let instr = program[ip];
        let op = program[ip + 1];
        ip += 2;
        let combo = match op {
            0..=3 => op,
            4 => a,
            5 => b,
            6 => c,
            _ => 0,
        };
        match instr {
            0 => a >>= combo,
            1 => b ^= op,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    ip = op;
                }
            }
            4 => b ^= c,
            5 => output.push(combo % 8),
            6 => b = a >> combo,
            7 => c = a >> combo,
            _ => panic!(),
        }
    }
    output
}

pub fn part1(input: &str) -> String {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut registers = registers
        .lines()
        .map(|reg| reg[12..].parse::<usize>().unwrap());
    let reg_a = registers.next().unwrap();
    let reg_b = registers.next().unwrap();
    let reg_c = registers.next().unwrap();

    let program: Vec<usize> = program[9..]
        .trim()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let machine_state = MachineState::new(reg_a, reg_b, reg_c, &program);
    machine_state
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part2(input: &str) -> usize {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut registers = registers
        .lines()
        .map(|reg| reg[12..].parse::<usize>().unwrap());
    let _ = registers.next().unwrap();
    let reg_b = registers.next().unwrap();
    let reg_c = registers.next().unwrap();

    let program: Vec<usize> = program[9..]
        .trim()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut reg_a = 0;
    let mut incr = 1;
    loop {
        let mut machine_state = MachineState::new(reg_a, reg_b, reg_c, &program);
        let mut output = program.iter().copied();
        if (&mut machine_state).zip(&mut output).all(|(a, b)| a == b) {
            if machine_state.next().is_none() && output.next().is_none() {
                return reg_a;
            } else {
                incr = next_multiple_of_two(reg_a);
            }
        }
        reg_a += incr;
    }
}

fn next_multiple_of_two(value: usize) -> usize {
    let mut multiple = 1;
    while multiple < value {
        multiple <<= 1;
    }
    multiple
}

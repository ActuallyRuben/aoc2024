use crate::util::Solver;

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

struct MachineSolver {
    solver: Solver,
}

impl MachineSolver {
    fn new() -> Self {
        let solver = Solver::new();
        Self { solver }
    }

    fn eval_program(&mut self, b: usize, c: usize, program: &[usize]) -> usize {
        let mut output = program;
        let original_a = self.solver.unknown();
        let mut a = original_a;
        let mut b = self.solver.known(b);
        let mut c = self.solver.known(c);
        let mut ip = 0;
        let bitmask = self.solver.known(0x7);
        let zero = self.solver.known(0);
        loop {
            if ip >= program.len() {
                break;
            }
            let instr = program[ip];
            let op = program[ip + 1];
            ip += 2;
            let operand = self.solver.known(op);
            let combo = match op {
                0..=3 => operand,
                4 => a,
                5 => b,
                6 => c,
                _ => operand,
            };
            match instr {
                0 => a = self.solver.assign(a >> combo),
                1 => b = self.solver.assign(b ^ operand),
                2 => b = self.solver.assign(combo & bitmask),
                3 => {
                    if output.is_empty() {
                        self.solver.constraint(a, zero.into());
                    } else {
                        ip = op;
                    }
                }
                4 => b = self.solver.assign(b ^ c),
                5 => {
                    let value = self.solver.known(output[0]);
                    self.solver.constraint(value, combo & bitmask);
                    output = &output[1..];
                }
                6 => b = self.solver.assign(a >> combo),
                7 => c = self.solver.assign(a >> combo),
                _ => panic!(),
            }
        }
        self.solver.solve_for(original_a)
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

#[allow(unused)]
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

#[allow(unused)]
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

    let mut solver = MachineSolver::new();
    solver.eval_program(reg_b, reg_c, &program)
}

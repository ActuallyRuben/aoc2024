use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Gate<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
    Constant(bool),
}

fn parse_input(input: &str) -> HashMap<&str, Gate> {
    let (inputs, gates_input) = input.split_once("\n\n").unwrap();
    let mut gates = HashMap::new();
    for input in inputs.lines() {
        let value = input.ends_with('1');
        gates.insert(&input[0..3], Gate::Constant(value));
    }

    for gate in gates_input.lines() {
        let (name, gate) = match gate.chars().nth(4).unwrap() {
            'X' => (&gate[15..18], Gate::Xor(&gate[0..3], &gate[8..11])),
            'A' => (&gate[15..18], Gate::And(&gate[0..3], &gate[8..11])),
            'O' => (&gate[14..17], Gate::Or(&gate[0..3], &gate[7..10])),
            _ => panic!("invalid input"),
        };
        gates.insert(name, gate);
    }
    gates
}

fn eval_gates<'a>(key: &str, gates: &mut HashMap<&'a str, Gate<'a>>) -> Option<bool> {
    let value = match *gates.get(key)? {
        Gate::And(a, b) => eval_gates(a, gates)? && eval_gates(b, gates)?,
        Gate::Or(a, b) => eval_gates(a, gates)? || eval_gates(b, gates)?,
        Gate::Xor(a, b) => eval_gates(a, gates)? ^ eval_gates(b, gates)?,
        Gate::Constant(value) => return Some(value),
    };
    *gates.get_mut(key).unwrap() = Gate::Constant(value);
    Some(value)
}

pub fn part1(input: &str) -> usize {
    let mut gates = parse_input(input);
    let mut output = 0;
    for i in 0..99 {
        let key = format!("z{i:02}");
        let Some(value) = eval_gates(&key, &mut gates) else {
            break;
        };
        if value {
            output |= 1 << i;
        }
    }
    output
}

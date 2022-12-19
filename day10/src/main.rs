use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

enum Operation {
    NOOP,
    ADDX(i32),
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(" ").collect();

        if tokens.len() == 1 && *tokens.get(0).unwrap() == "noop" {
            Ok(Operation::NOOP)
        } else if tokens.len() == 2 && *tokens.get(0).unwrap() == "addx" {
            Ok(Operation::ADDX(
                i32::from_str_radix(tokens.get(1).unwrap(), 10).unwrap(),
            ))
        } else {
            Err("Unknown operation".to_owned())
        }
    }
}

fn load_operations(path: &'static str) -> Vec<Operation> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut operations: Vec<Operation> = Vec::new();

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let operation = Operation::from_str(line.as_str()).unwrap();
            operations.push(operation);
        }
    }

    operations
}

fn simulate(operations: &Vec<Operation>, coi: &Vec<i32>) -> Vec<i32> {
    let mut x_register: i32 = 1;
    let mut instruction_pointer: usize = 0;
    let mut cycle = 0;

    let mut delta = 0;

    let mut signal_strengths: Vec<i32> = Vec::new();

    while instruction_pointer != operations.len() {
        if x_register.abs_diff(cycle % 40) <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if (cycle + 1) % 40 == 0 {
            println!();
        }
        cycle += 1;

        if coi.contains(&cycle) {
            signal_strengths.push(cycle * x_register);
        }

        if delta != 0 {
            x_register += delta;
            delta = 0;
            instruction_pointer += 1;
        } else {
            match operations[instruction_pointer] {
                Operation::NOOP => {
                    instruction_pointer += 1;
                }
                Operation::ADDX(x) => {
                    delta = x;
                }
            }
        }
    }

    signal_strengths
}

fn main() {
    let operations = load_operations("./day10/input.txt");

    println!(
        "Sum of signal strengths: {}",
        simulate(&operations, &vec![20, 60, 100, 140, 180, 220])
            .iter()
            .sum::<i32>()
    );
}

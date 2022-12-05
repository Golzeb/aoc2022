use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

type Stack<T> = Vec<T>;

fn load_stacks_with_moves(path: &'static str) -> (Vec<Stack<char>>, Vec<Move>) {
    let mut stacks: Vec<Stack<char>> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut first = true;
    let mut stack_data = true;

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            if line == "" || (!line.contains("[") && !line.contains("move")) {
                stack_data = false;
                continue;
            }

            if stack_data {
                let chars = line.chars().collect::<Vec<char>>();

                if first {
                    first = false;
                    let length = (line.chars().count() + 1) / 4;

                    stacks.resize(length, Stack::new());
                }

                let stack_values = chars.chunks(4).map(|e| e[1]).collect::<Vec<char>>();

                for (index, element) in stack_values.iter().enumerate() {
                    if *element != ' ' {
                        stacks[index].push(*element);
                    }
                }
            } else {
                let tokens = line.split(" ").collect::<Vec<&str>>();

                moves.push(Move {
                    amount: usize::from_str_radix(tokens[1], 10).unwrap(),
                    from: usize::from_str_radix(tokens[3], 10).unwrap() - 1,
                    to: usize::from_str_radix(tokens[5], 10).unwrap() - 1,
                });
            }
        }
    }

    stacks = stacks
        .into_iter()
        .map(|mut e| {
            e.reverse();
            e
        })
        .collect::<Vec<Stack<char>>>();

    (stacks, moves)
}

fn get_crates_on_top_cm9000(stacks: &Vec<Stack<char>>, moves: &Vec<Move>) -> String {
    let mut temp_stacks = stacks.clone();

    for m in moves {
        for _ in 0..m.amount {
            if let Some(c) = temp_stacks[m.from].pop() {
                temp_stacks[m.to].push(c);
            }
        }
    }

    let mut out = String::new();

    for stack in &temp_stacks {
        if let Some(value) = stack.last() {
            out.push(*value);
        }
    }

    out
}

fn get_crates_on_top_cm9001(stacks: &Vec<Stack<char>>, moves: &Vec<Move>) -> String {
    let mut temp_stacks = stacks.clone();

    for m in moves {
        let mut moved = temp_stacks[m.from]
            .iter()
            .rev()
            .take(m.amount)
            .rev()
            .copied()
            .collect::<Vec<char>>();

        temp_stacks[m.to].append(&mut moved);
        for _ in 0..m.amount {
            temp_stacks[m.from].pop();
        }
    }

    let mut out = String::new();

    for stack in &temp_stacks {
        if let Some(value) = stack.last() {
            out.push(*value);
        }
    }

    out
}

fn main() {
    let (stacks, moves) = load_stacks_with_moves("./day5/input.txt");

    println!(
        "Crates on top (CrateMover 9000): {}",
        get_crates_on_top_cm9000(&stacks, &moves)
    );
    println!(
        "Crates on top (CrateMover 9001): {}",
        get_crates_on_top_cm9001(&stacks, &moves)
    );
}

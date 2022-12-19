use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

enum Move {
    UP(i32),
    DOWN(i32),
    LEFT(i32),
    RIGHT(i32),
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(" ").collect();

        if tokens.len() != 2 {
            return Err("Wrong format".to_owned());
        }

        match *tokens.get(0).unwrap() {
            "U" => Ok(Move::UP(
                i32::from_str_radix(tokens.get(1).unwrap(), 10).unwrap(),
            )),
            "D" => Ok(Move::DOWN(
                i32::from_str_radix(tokens.get(1).unwrap(), 10).unwrap(),
            )),
            "L" => Ok(Move::LEFT(
                i32::from_str_radix(tokens.get(1).unwrap(), 10).unwrap(),
            )),
            "R" => Ok(Move::RIGHT(
                i32::from_str_radix(tokens.get(1).unwrap(), 10).unwrap(),
            )),
            _ => Err("Unknown move".to_owned()),
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn load_moves(path: &'static str) -> Vec<Move> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut moves: Vec<Move> = Vec::new();

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let move_info = Move::from_str(line.as_str()).unwrap();
            moves.push(move_info);
        }
    }

    moves
}

fn count_unique_tail_positions(moves: &Vec<Move>, length: i32) -> i32 {
    let mut knots_positions: Vec<Position> = Vec::new();

    for _ in 0..(length + 1) {
        knots_positions.push(Position::default());
    }

    let mut tail_positions: Vec<Position> = vec![*knots_positions.last().unwrap()];

    for move_info in moves {
        let distance = match move_info {
            Move::UP(d) => *d,
            Move::DOWN(d) => *d,
            Move::LEFT(d) => *d,
            Move::RIGHT(d) => *d,
        };

        for _ in 0..distance {
            match move_info {
                Move::UP(_) => knots_positions.first_mut().unwrap().y += 1,
                Move::DOWN(_) => knots_positions.first_mut().unwrap().y -= 1,
                Move::LEFT(_) => knots_positions.first_mut().unwrap().x -= 1,
                Move::RIGHT(_) => knots_positions.first_mut().unwrap().x += 1,
            };

            for knot_index in 1..knots_positions.len() {
                let delta_x: i32;
                let delta_y: i32;

                {
                    let previous = knots_positions.get(knot_index - 1).unwrap();
                    let current = knots_positions.get(knot_index).unwrap();

                    if previous.x.abs_diff(current.x) > 1 || previous.y.abs_diff(current.y) > 1 {
                        delta_x = (previous.x - current.x).clamp(-1, 1);
                        delta_y = (previous.y - current.y).clamp(-1, 1);
                    } else {
                        delta_x = 0;
                        delta_y = 0;
                    }
                }

                let mut current = knots_positions.get_mut(knot_index).unwrap();

                current.x += delta_x;
                current.y += delta_y;
            }

            if !tail_positions.contains(&knots_positions.last().unwrap()) {
                tail_positions.push(*knots_positions.last().unwrap());
            }
        }
    }

    tail_positions.len() as i32
}

fn main() {
    let moves = load_moves("./day9/input.txt");

    println!(
        "Unique tail positions: {}",
        count_unique_tail_positions(&moves, 1)
    );

    println!(
        "Unique tail positions with 9 knots: {}",
        count_unique_tail_positions(&moves, 9)
    );
}

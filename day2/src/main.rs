use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn get_shape(&self, expected_outcome: RoundOutcome) -> Shape {
        match self {
            Shape::Rock => match expected_outcome {
                RoundOutcome::Win => Shape::Paper,
                RoundOutcome::Draw => Shape::Rock,
                RoundOutcome::Lose => Shape::Scissors,
            },
            Shape::Paper => match expected_outcome {
                RoundOutcome::Win => Shape::Scissors,
                RoundOutcome::Draw => Shape::Paper,
                RoundOutcome::Lose => Shape::Rock,
            },
            Shape::Scissors => match expected_outcome {
                RoundOutcome::Win => Shape::Rock,
                RoundOutcome::Draw => Shape::Scissors,
                RoundOutcome::Lose => Shape::Paper,
            },
        }
    }
}

impl TryFrom<&str> for Shape {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err("Unknown shape"),
        }
    }
}

#[derive(Copy, Clone)]
enum RoundOutcome {
    Win,
    Draw,
    Lose,
}

impl TryFrom<&str> for RoundOutcome {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(RoundOutcome::Lose),
            "Y" => Ok(RoundOutcome::Draw),
            "Z" => Ok(RoundOutcome::Win),
            _ => Err("Unknown round result"),
        }
    }
}

fn load_strategy() -> Vec<(Shape, (Shape, Shape))> {
    let mut out: Vec<(Shape, (Shape, Shape))> = Vec::new();

    let file = File::open("./day2/input.txt").unwrap();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let game = line.split(" ").collect::<Vec<&str>>();

            let opponent = Shape::try_from(game[0]).unwrap();
            let me = Shape::try_from(game[1]).unwrap();
            let outcome = RoundOutcome::try_from(game[1]).unwrap();

            out.push((opponent, (me, opponent.get_shape(outcome))));
        }
    }

    out
}

fn main() {
    let strategy = load_strategy();

    let mut original_points = 0;
    let mut new_points = 0;
    for (a, b) in &strategy {
        original_points += match (b.0 as i32) - (*a as i32) {
            -2 => 6,
            0 => 3,
            1 => 6,
            _ => 0,
        };

        new_points += match (b.1 as i32) - (*a as i32) {
            -2 => 6,
            0 => 3,
            1 => 6,
            _ => 0,
        };

        original_points += b.0 as i32;
        new_points += b.1 as i32;
    }

    println!("First strategy score: {}", original_points);
    println!("Second strategy score: {}", new_points);
}

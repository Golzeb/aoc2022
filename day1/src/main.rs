use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

struct Elf {
    calories: Vec<i32>,
}

fn load_elves(path: &'static str) -> Vec<Elf> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut elves: Vec<Elf> = Vec::new();

    let mut temp_vector: Vec<i32> = Vec::new();
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            if line != "" {
                let calories = i32::from_str_radix(line.as_str(), 10).unwrap();

                temp_vector.push(calories);
            } else {
                elves.push(Elf {
                    calories: temp_vector,
                });

                temp_vector = Vec::new();
            }
        }
    }

    elves
}

fn main() {
    let elves = load_elves("./day1/input.txt");

    println!(
        "Elf carrying the most calories carries {} calories",
        elves
            .iter()
            .map(|e| e.calories.iter().sum::<i32>())
            .max()
            .unwrap()
    );

    println!(
        "Top three elves carrying the most calories carry {} calories",
        elves
            .iter()
            .map(|e| e.calories.iter().sum::<i32>())
            .collect::<BinaryHeap<i32>>()
            .iter()
            .take(3)
            .sum::<i32>()
    );
}

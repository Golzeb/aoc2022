use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
}

impl Rucksack {
    fn get_shared_items(&self) -> Vec<char> {
        let mut shared_items: Vec<char> = Vec::new();

        for item in &self.first_compartment {
            if self.second_compartment.contains(&item) && !shared_items.contains(&item) {
                shared_items.push(*item);
            }
        }

        shared_items
    }
}

impl From<&str> for Rucksack {
    fn from(value: &str) -> Self {
        let len = value.len();

        let chars = value.chars().collect::<Vec<char>>();

        let chunks = chars.chunks(len / 2).collect::<Vec<_>>();

        Rucksack {
            first_compartment: chunks[0].to_vec(),
            second_compartment: chunks[1].to_vec(),
        }
    }
}

fn get_badge(a: &Rucksack, b: &Rucksack, c: &Rucksack) -> char {
    for item in a
        .first_compartment
        .iter()
        .chain(a.second_compartment.iter())
    {
        if let Some(_) = b
            .first_compartment
            .iter()
            .chain(b.second_compartment.iter())
            .find(|&e| e == item)
        {
            if let Some(_) = c
                .first_compartment
                .iter()
                .chain(c.second_compartment.iter())
                .find(|&e| e == item)
            {
                return *item;
            }
        }
    }

    ' '
}

fn get_priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 96,
        'A'..='Z' => item as i32 - 38,
        _ => 0,
    }
}

fn get_priority_sum(rucksacks: &Vec<Rucksack>) -> i32 {
    let mut sum = 0;

    for rucksack in rucksacks {
        let shared_items = rucksack.get_shared_items();

        for shared_item in shared_items {
            sum += get_priority(shared_item);
        }
    }

    sum
}

fn get_badge_priority_sum(rucksacks: &Vec<Rucksack>) -> i32 {
    let mut sum = 0;

    for chunk in rucksacks.chunks(3) {
        sum += get_priority(get_badge(&chunk[0], &chunk[1], &chunk[2]));
    }

    sum
}

fn load_rucksacks(path: &'static str) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            rucksacks.push(Rucksack::from(line.as_str()));
        }
    }

    rucksacks
}

fn main() {
    let rucksacks = load_rucksacks("./day3/input.txt");

    println!("Sum of priorities: {}", get_priority_sum(&rucksacks));
    println!(
        "Sum of badge priorities: {}",
        get_badge_priority_sum(&rucksacks)
    );
}

#[test]
fn test_example() {
    let data = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for rucksack_data in &data {
        rucksacks.push(Rucksack::from(*rucksack_data));
    }

    assert_eq!(
        get_priority_sum(&rucksacks),
        157,
        "get_priority_sum wrong sum"
    );

    assert_eq!(
        get_badge_priority_sum(&rucksacks),
        70,
        "get_badges_priority_sum wrong sum"
    );
}

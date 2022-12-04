use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    process::Output,
};

struct Assignment(RangeInclusive<i32>, RangeInclusive<i32>);

fn get_range_from_str(value: &str) -> RangeInclusive<i32> {
    let values = value
        .split("-")
        .map(|e| i32::from_str_radix(e, 10).unwrap())
        .collect::<Vec<i32>>();

    values[0]..=values[1]
}

fn load_assignments(path: &'static str) -> Vec<Assignment> {
    let mut assignments: Vec<Assignment> = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let values = line.split(",").collect::<Vec<&str>>();
            assignments.push(Assignment {
                0: get_range_from_str(values[0]),
                1: get_range_from_str(values[1]),
            });
        }
    }

    assignments
}

fn get_number_of_fully_contained_assignments(assignments: &Vec<Assignment>) -> i32 {
    let mut out = 0;

    for assignment in assignments {
        let start = assignment.0.start().min(assignment.1.start());
        let end = assignment.0.end().max(assignment.1.end());
        let range = (*start)..=(*end);
        if assignment.0 == range || assignment.1 == range {
            out += 1;
        }
    }

    out
}

fn get_number_of_overlapping_assignments(assignments: &Vec<Assignment>) -> i32 {
    let mut out = 0;

    for assignment in assignments {
        let start = assignment.0.start().min(assignment.1.start());
        let end = assignment.0.end().max(assignment.1.end());
        let range = (*start)..=(*end);

        let range_length = range.count();
        let length0 = assignment.0.clone().count();
        let length1 = assignment.1.clone().count();

        if range_length < length0 + length1 {
            out += 1;
        }
    }

    out
}

fn main() {
    let assignments = load_assignments("./day4/input.txt");

    println!(
        "Number of fully contained ranges: {}",
        get_number_of_fully_contained_assignments(&assignments)
    );

    println!(
        "Number of overlapping ranges: {}",
        get_number_of_overlapping_assignments(&assignments)
    );
}

#[test]
fn test_example() {
    let data = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];

    let mut assignments: Vec<Assignment> = Vec::new();

    for assignments_data in &data {
        let values = assignments_data.split(",").collect::<Vec<&str>>();
        assignments.push(Assignment {
            0: get_range_from_str(values[0]),
            1: get_range_from_str(values[1]),
        });
    }

    assert_eq!(
        get_number_of_fully_contained_assignments(&assignments),
        2,
        "get_number_of_fully_contained_assignments"
    );

    assert_eq!(
        get_number_of_overlapping_assignments(&assignments),
        4,
        "get_number_of_overlapping_assignments"
    );
}

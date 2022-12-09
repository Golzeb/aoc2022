use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct CellInfo {
    max_top: i8,
    max_bottom: i8,
    max_left: i8,
    max_right: i8,
    value: i8,
}

impl CellInfo {
    fn new(value: i8) -> Self {
        CellInfo {
            max_top: -1,
            max_bottom: -1,
            max_left: -1,
            max_right: -1,
            value,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn load_cells(path: &'static str) -> (HashMap<Position, CellInfo>, usize, usize) {
    let mut cells: HashMap<Position, CellInfo> = HashMap::new();
    let mut array: Vec<Vec<i8>> = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            array.push(
                line.chars()
                    .map(|e| e.to_digit(10).unwrap() as i8)
                    .collect::<Vec<i8>>(),
            );
        }
    }

    for i in 0..array[0].len() {
        for j in 0..array.len() {
            cells.insert(Position::new(i, j), CellInfo::new(array[j][i]));
        }
    }

    for i in 1..array[0].len() - 1 {
        for j in 1..array.len() - 1 {
            let position = Position::new(i, j);
            let position_top = Position::new(i, j - 1);
            let position_left = Position::new(i - 1, j);

            let new_left = {
                let cell_left = cells.get(&position_left).unwrap();
                cell_left.max_left.max(cell_left.value)
            };
            let new_top = {
                let cell_top = cells.get(&position_top).unwrap();
                cell_top.max_top.max(cell_top.value)
            };

            let mut cell = cells.get_mut(&position).unwrap();

            cell.max_left = new_left;
            cell.max_top = new_top;
        }
    }

    for i in (1..(array[0].len() - 1)).rev() {
        for j in (1..(array.len() - 1)).rev() {
            let position = Position::new(i, j);
            let position_right = Position::new(i + 1, j);
            let position_bottom = Position::new(i, j + 1);

            let new_right = {
                let cell_right = cells.get(&position_right).unwrap();
                cell_right.max_right.max(cell_right.value)
            };
            let new_bottom = {
                let cell_bottom = cells.get(&position_bottom).unwrap();
                cell_bottom.max_bottom.max(cell_bottom.value)
            };

            let mut cell = cells.get_mut(&position).unwrap();
            cell.max_bottom = new_bottom;
            cell.max_right = new_right;
        }
    }

    (cells, array[0].len(), array.len())
}

fn get_number_of_visible_tress(cells: &HashMap<Position, CellInfo>) -> u32 {
    let mut count = 0;
    for (_, v) in cells.iter() {
        if v.value > v.max_top
            || v.value > v.max_bottom
            || v.value > v.max_left
            || v.value > v.max_right
        {
            count += 1;
        }
    }
    count
}

fn get_highest_scenic_score(cells: &HashMap<Position, CellInfo>, width: u32, height: u32) -> u32 {
    let mut highest_score = 0;

    for (k, v) in cells.iter() {
        if k.x as u32 != 0 && k.x as u32 != width - 1 && k.y as u32 != 0 && k.y as u32 != height - 1
        {
            let distance_left = {
                let mut position_x = k.x - 1;
                let mut distance = 1;

                while position_x > 0 {
                    let cell = cells.get(&Position::new(position_x, k.y)).unwrap();

                    if cell.value < v.value {
                        distance += 1;
                    } else {
                        break;
                    }

                    position_x -= 1;
                }

                distance
            };

            let distance_right = {
                let mut position_x = k.x + 1;
                let mut distance = 1;

                while position_x < width as usize - 1 {
                    let cell = cells.get(&Position::new(position_x, k.y)).unwrap();

                    if cell.value < v.value {
                        distance += 1;
                    } else {
                        break;
                    }

                    position_x += 1;
                }

                distance
            };

            let distance_top = {
                let mut position_y = k.y - 1;
                let mut distance = 1;

                while position_y > 0 {
                    let cell = cells.get(&Position::new(k.x, position_y)).unwrap();

                    if cell.value < v.value {
                        distance += 1;
                    } else {
                        break;
                    }

                    position_y -= 1;
                }

                distance
            };

            let distance_bottom = {
                let mut position_y = k.y + 1;
                let mut distance = 1;

                while position_y < height as usize - 1 {
                    let cell = cells.get(&Position::new(k.x, position_y)).unwrap();

                    if cell.value < v.value {
                        distance += 1;
                    } else {
                        break;
                    }

                    position_y += 1;
                }

                distance
            };

            let score = distance_bottom * distance_top * distance_left * distance_right;

            highest_score = highest_score.max(score);
        }
    }

    highest_score
}

fn main() {
    let (cells, width, height) = load_cells("./day8/input.txt");

    println!(
        "Number of visible trees: {}",
        get_number_of_visible_tress(&cells)
    );

    println!(
        "Highest sceninc score: {}",
        get_highest_scenic_score(&cells, width as u32, height as u32)
    );
}

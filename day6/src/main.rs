use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

fn load_data(path: &'static str) -> String {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    buf
}

fn find_marker_of_length(data: &String, length: i32) -> i32 {
    let slice = &data.as_bytes()[0..];

    for (index, window) in slice.windows(length as usize).enumerate() {
        let mut map: HashMap<u8, i32> = HashMap::new();

        for w in window {
            if !map.contains_key(w) {
                map.insert(*w, 1);
            } else {
                *map.get_mut(w).unwrap() += 1;
            }
        }

        if map.values().all(|&e| e == 1) {
            return index as i32 + length;
        }
    }

    0
}

fn main() {
    let data = load_data("./day6/input.txt");

    println!(
        "Bytes processed before the first start-of-packer marker: {}",
        find_marker_of_length(&data, 4)
    );

    println!(
        "Bytes processed before the first start-of-message marker: {}",
        find_marker_of_length(&data, 14)
    );
}

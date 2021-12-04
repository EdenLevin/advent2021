use std::{
    fs::File,
    io::{self, BufRead},
};

struct Position {
    depth: i32,
    horizontal: i32,
}
fn calculate_position_aim(file: File) -> Position {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    let reader = io::BufReader::new(file).lines();
    for line in reader.flatten() {
        let mut parts = line.split_ascii_whitespace();
        let direction = parts.next().unwrap();
        let amount = parts.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => unreachable!(),
        }
    }

    Position { depth, horizontal }
}

fn calculate_position(file: File) -> Position {
    let mut depth = 0;
    let mut horizontal = 0;

    let reader = io::BufReader::new(file).lines();
    for line in reader.flatten() {
        let mut parts = line.split_ascii_whitespace();
        let direction = parts.next().unwrap();
        let amount = parts.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => unreachable!(),
        }
    }

    Position { depth, horizontal }
}

pub fn calculate_product(file: File) -> i32 {
    let Position { depth, horizontal } = calculate_position(file);
    depth * horizontal
}
pub fn calculate_product_aim(file: File) -> i32 {
    let Position { depth, horizontal } = calculate_position_aim(file);
    depth * horizontal
}

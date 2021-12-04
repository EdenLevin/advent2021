use std::{
    fs::File,
    io::{self, BufRead, Read},
};

pub fn count_increases(file: File) -> u32 {
    let reader = io::BufReader::new(file).lines();
    let mut prev_value = i32::MAX;
    let mut count = 0;
    for line in reader.flatten() {
        let num = line.parse::<i32>().unwrap();
        if num > prev_value {
            count += 1;
        }
        prev_value = num;
    }
    count
}
pub fn count_sliding_window_increases(file: File) -> u32 {
    let mut file = file;
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut vec = vec![];
    for line in buf.lines() {
        let num = line.parse::<i32>().unwrap();
        vec.push(num);
    }

    let mut prev_value = i32::MAX;
    let mut count = 0;

    for nums in vec.windows(3) {
        let current: i32 = nums.iter().sum();
        if current > prev_value {
            count += 1;
        }
        prev_value = current;
    }
    count
}

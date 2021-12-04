use std::{
    fs::File,
    io::{self, BufRead},
};

const BITS: usize = 12;

fn calculate_gamma(file: File) -> u32 {
    let mut count = [0; BITS];
    let mut line_count = 0;

    let reader = io::BufReader::new(file).lines();
    for line in reader.flatten() {
        line_count += 1;
        let amount = u32::from_str_radix(&line, 2).unwrap();
        for (i, item) in count.iter_mut().enumerate().take(BITS) {
            *item += (amount >> (BITS - 1 - i)) & 1;
        }
    }
    let mut ans = 0;
    for item in count.iter().take(BITS) {
        ans <<= 1;
        ans |= if item * 2 > line_count { 1 } else { 0 };
    }
    ans
}

pub fn calculate_power_consumption(file: File) -> u32 {
    let gamma = calculate_gamma(file);
    let epsilon = (!gamma) & ((1 << BITS) - 1);

    gamma * epsilon
}

fn find_majority_bit(vec: &[u32], bit: usize) -> u32 {
    let mut count = 0;
    let mut one_count = 0;
    for val in vec {
        count += 1;
        one_count += (val >> bit) & 1;
    }
    if one_count * 2 >= count {
        1
    } else {
        0
    }
}

struct LifeSupportRating {
    co2: u32,
    oxygen: u32,
}

fn calculate_oxygen(file: File) -> LifeSupportRating {
    let mut values = vec![];

    let reader = io::BufReader::new(file).lines();
    for line in reader.flatten() {
        let amount = u32::from_str_radix(&line, 2).unwrap();
        values.push(amount);
    }

    let mut values2 = values.clone();
    for i in (0..BITS).rev() {
        let bit = find_majority_bit(&values, i);
        values.retain(|num| (num >> i) & 1 == bit);
    }
    for i in (0..BITS).rev() {
        let bit = 1 - find_majority_bit(&values2, i);
        values2.retain(|num| (num >> i) & 1 == bit);
        if values2.len() == 1 {
            break;
        }
    }

    LifeSupportRating {
        co2: values2[0],
        oxygen: values[0],
    }
}

pub fn calculate_life_support(file: File) -> u32 {
    let LifeSupportRating { co2, oxygen } = calculate_oxygen(file);

    co2 * oxygen
}

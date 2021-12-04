use std::fs::File;

pub mod day1;
pub mod day2;
mod day3;
mod day4;
fn main() {
    let path = "rsc\\day1.txt";
    let count = day1::count_increases(File::open(path).unwrap());
    println!("day1 q1 is: {}", count);
    let count = day1::count_sliding_window_increases(File::open(path).unwrap());
    println!("day1 q2 is: {}", count);

    let path = "rsc\\day2.txt";
    let ans = day2::calculate_product(File::open(path).unwrap());
    println!("day2 q1 is: {}", ans);
    let ans = day2::calculate_product_aim(File::open(path).unwrap());
    println!("day2 q2 is: {}", ans);

    let path = "rsc\\day3.txt";
    let ans = day3::calculate_power_consumption(File::open(path).unwrap());
    println!("day3 q1 is: {}", ans);
    let ans = day3::calculate_life_support(File::open(path).unwrap());
    println!("day3 q2 is: {}", ans);

    let path = "rsc\\day4.txt";
    let ans = day4::play_bingo_to_win(File::open(path).unwrap());
    println!("day4 q1 is: {}", ans);
    let ans = day4::play_bingo_to_lose(File::open(path).unwrap());
    println!("day4 q2 is: {}", ans);
}

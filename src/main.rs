use std::fs::File;

pub mod day1;
pub mod day2;
fn main() {
    let path = "rsc\\day1.txt";
    let count = day1::count_increases(File::open(path).unwrap());
    println!("day1 q1 is :{}", count);
    let count = day1::count_sliding_window_increases(File::open(path).unwrap());
    println!("day1 q2 is :{}", count);

    let path = "rsc\\day2.txt";
    let ans = day2::calculate_product(File::open(path).unwrap());
    println!("day2 q1 is :{}", ans);
    let ans = day2::calculate_product_aim(File::open(path).unwrap());
    println!("day2 q2 is :{}", ans);
}

use std::fs::File;

pub mod day1;
fn main() {
    let path = "rsc\\day1.txt";
    let count = day1::count_increases(File::open(path).unwrap());
    println!("count is :{}", count);
    let count = day1::count_sliding_window_increases(File::open(path).unwrap());
    println!("window count is :{}", count);
}

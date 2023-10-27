use clap::Parser;
use hashmap_count::logic;
use std::env;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}


fn main() {
    let input = env::args().skip(1).next().unwrap_or_default();
    let numbers: Vec<i32> = input
    .split(',')
    .map(|s| s.trim())
    .map(|s| s.parse().unwrap())
    .collect();
    let result = logic(numbers);
    println!("Result: {:?}", result);
}

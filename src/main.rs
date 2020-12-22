use advent_code_2020::solve_day;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = match args.get(1) {
        Some(x) => x.parse::<usize>().unwrap(),
        None => 16,
    };

    println!("Running Advent of Code {{2020}} day: {}", day);
    solve_day(day);
}
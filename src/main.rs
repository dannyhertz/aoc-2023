mod day1;
mod day2;

use std::fs;

fn main() {
    // Read input data from a file or any other source
    let input = fs::read_to_string("input/day2.txt").expect("Error reading input file");


    // Print the result
    println!("Day 2 Solution Part I: {}", day2::solve(&input));
    println!("Day 2 Solution Part II: {}", day2::solve_2(&input));
}

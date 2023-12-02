mod day1;

use std::fs;

fn main() {
    // Read input data from a file or any other source
    let input = fs::read_to_string("input/day1.txt").expect("Error reading input file");

    // Call the solve function for Day 1
    let result = day1::solve(&input);

    // Print the result
    println!("Day 1 Solution: {}", result);
}

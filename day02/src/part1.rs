mod common;
use common::*;
use std::fs::read_to_string;

/**
 * Reads input.txt, which contains lines of several integers separated by spaces.
 * A 'safe' line has all of the following conditions:
 * 
 *   - The values are all increasing or decreasing
 *   - Two adjacent values differ by at least 1 and at most 3
 * 
 * Prints the number of 'safe' lines.
 */
fn main() {
    
    let mut safe_lines = 0;

    // Read all the lines in the file
    for line in read_to_string("input.txt").unwrap().lines() {

        // Split the line into values
        let levels = line.split_ascii_whitespace();

        // Map the values into integers
        // & collect them for ease of processing
        let int_levels: Vec<i32> = levels.map(
            |col| -> i32 { col.parse().expect("Found a noninteger level") }
        ).collect();

        // Check the line
        match check(&int_levels) {
            Safety::Safe => safe_lines += 1,
            Safety::Unsafe(_unsafe_reason) => continue,
        }
    }

    println!("{}", safe_lines);

}
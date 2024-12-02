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
 * A line may also be made safe by removal of a single value.
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

        // Check the line without any removals first
        // If it's already fine, move on. Otherwise, try removing entries.
        if check(&int_levels) == Safety::Safe {
            safe_lines += 1;
            continue;
        }

        for index in 0..int_levels.len() {

            // Remove the item at this index
            let mut int_levels_with_removal = int_levels.clone();
            int_levels_with_removal.remove(index);

            // Re-check
            if check(&int_levels_with_removal) == Safety::Safe {
                safe_lines += 1;

                // No need to try more removals
                break;
            }
        }
    }

    println!("{}", safe_lines);

}

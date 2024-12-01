use std::fs::read_to_string;

/**
 * Reads input.txt, which contains two ASCII whitespace sparated columns of integers.
 * Sorts the columns independently and totalises the absolute difference the columns for each row.
 */
fn main() {
    
    // Read the input file line by line, populating our two lists
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in read_to_string("input.txt").unwrap().lines() {

        // Split into columns
        let columns_iter = line.split_ascii_whitespace();

        // Map them into integers
        let mut int_columns = 
            columns_iter.map(
                |col| -> u32 { col.parse().expect("Found a noninteger value") }
            ); 

        list_a.push(int_columns.next().expect("Missing column A"));
        list_b.push(int_columns.next().expect("Missing column B"));
    }

    // Assert that the lists hvae the same length
    assert_eq!(list_a.len(), list_b.len());
    
    // Sort the two lists
    list_a.sort();
    list_b.sort();

    // Zip the two lists
    let zip_iter = list_a.iter().zip(list_b.iter());

    // Sum the columns
    let total: u32 = zip_iter
        .map(|pair| -> u32 { pair.0.abs_diff(*(pair.1)) } )
        .sum()
    ;    

    println!("{}", total);

}

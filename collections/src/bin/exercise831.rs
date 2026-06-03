use std::collections::HashMap;

fn main() {
    // GOAL: Given a list of integers, calculate and return the median and mode
    let mut integers: Vec<i32> = vec![3, 3, 4, 3, 4, 5, 2, 4, 1, 6]; // unsorted vector (mut to sort)
    integers.sort(); // sorted vector

    // Calculating the median
    println!("Median: {}", get_median(&integers));

    // Calculating the mode
    println!("Mode(s): {:?}", get_mode(&integers));
}

fn get_median(integers: &Vec<i32>) -> f64 {
    if integers.len() % 2 == 0 {
        // Case 1: Even

        // Implements Copy trait so does not need to be a reference
        // First index (left of the middle)
        let a: i32 = integers[(integers.len() / 2) - 1];
        // Second index (right of the middle)
        let b: i32 = integers[integers.len() / 2];

        // // You NEED the return keyword if you are EXITING A FUNCTION EARLY
        // return result;

        // But we can opt for highly idiomatic Rust instead (expression-oriented language)
        // wherein we use expressions more than the return keyword
        (a + b) as f64 / 2.0
    } else {
        // Case 2: Odd

        // Find the index of the median by dividing the vector's length by 2
        let index: usize = integers.len() / 2;
        // Access and return a reference to the median value in the vector
        integers[index] as f64 // implements the Copy trait so this is okay
    }
}

fn get_mode(integers: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut mode_map: HashMap<i32, i32> = HashMap::new();

    // Generates the mode map
    for int in integers {
        // Dereference because iterating over &Vec<i32> yields &i32
        let count: &mut i32 = mode_map.entry(*int).or_insert(0);
        *count += 1;
    }

    let mut max_keys: Vec<(i32, i32)> = Vec::new(); // wrapped in tuple for k-v pair
    let mut highest_count: i32 = 0; // tracks highest score

    // Find the mode(s)
    for (number, count) in &mode_map {
        // Iterate to find the ones with the highest values

        if *count < highest_count {
            // Case 1: Loser
            // Do nothing as it is not a mode
        } else if *count == highest_count {
            // Case 2: Tie
            // This integer appeared as many times as the current leader
            max_keys.push((*number, *count)); // deref because it's &i32's
        } else {
            // Case 3: Winner
            // This integer appeared more times than anything else before it
            // Now, the entire max_keys vector is full of losers, so update the
            // highest_count to be this new *count
            highest_count = *count;
            max_keys.clear(); // cleans vector
            max_keys.push((*number, *count)); // deref because it's &i32's
        }
    }

    max_keys
}

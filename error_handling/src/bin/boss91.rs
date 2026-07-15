fn main() {
    // Initialize vector
    let coords: Vec<i32> = vec![5, 10, 15];
    let index: usize = 3;

    // // Idiomatic Solution but not the requirement
    // // Check if valid
    // let destination = match coords.get(index) {
    //     Some(val) => {
    // println!("Success! Moved to {}.", { val });
    // val
    //     }
    //     None => panic!(
    //         "Cannot access places beyond world boundaries. The length of the world is {} and the requested index is {}.",
    //         coords.len(),
    //         index,
    //     ),
    // };

    // println!("Destination: {:?}", destination); // if Some, print a value, else, never called

    if index < coords.len() {
        // usize (unsigned) cannot be negative, so no need to check if >= 0
        // index needs to be positive as well
        println!("Success! Moved to {}.", coords[index]);
    } else {
        panic!(
            "Cannot access places beyond world boundaries. The length of the world is {} and the requested index is {}.",
            coords.len(),
            index,
        )
    }
}

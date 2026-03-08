use std::io;

fn main() {
    loop {
        // Ask user for n
        let mut n: String = String::new();
        println!("Please enter n: ");
        io::stdin().read_line(&mut n).expect("Failed to read line.");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,     // Return n if it's a valid u32
            Err(_) => continue, // Jump back to start of main loop on bad input
        };

        // Second match that checks for edge cases to decide whether to loop or not
        match n {
            0 => println!("0"),
            1 => println!("1"),
            _ => {
                // Core structure
                // Initialization happens inside to reset a and b values
                let mut a: u128 = 0; // Will be updated in loop
                let mut b: u128 = 1; // Will be updated in loop

                // Print initial variables
                println!("{a}");
                println!("{b}");

                // _ is "discard" as we only care about how many times it loops, not its index
                // Range is exclusive of upper bound (1 to n-1)
                for _ in 1..n {
                    // Find next term
                    let next_term: u128 = a + b;

                    // Print next term
                    println!("{next_term}");

                    // Update a and b to move forward 1 time
                    a = b;
                    b = next_term;
                }
            }
        }
    }
}

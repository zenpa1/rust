use std::io; // import I/O library from standard library

use rand::Rng; // Import Rng from rand

use std::cmp::Ordering; // Ordering is another enum with variants Less, Greater, and Equal

fn main() {
    let _apples = 5; // In Rust, variables are immutable by default
    // _ suppresses the compiler warning on unused variables

    println!("Guess the number!"); // A macro that prints a string

    // Loop for infinity
    loop {
        // Generate random secret number
        let secret_number = rand::thread_rng().gen_range(1..=100);
        // println!("The secret number is: {secret_number}");

        let mut guess = String::new(); // Create a variable to store user input

        // PUT THIS RIGHT AFTER THE LET MUT GUESS BECAUSE IT NEEDS TO READ A STRING
        io::stdin()
            .read_line(&mut guess) /* Calls the read_line method on the standard input handle
            &mut guess is an argument for read_line to tell it what string to store the input in */
            .expect("Failed to read line...");

        // But this is inferred as a String, so annotate the type by shadowing
        // let guess: u32 = guess.trim().parse().expect("Please type a number:");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // Match the trimmed and parsed value with the condition
        // Parse returns Result with the variants Ok and Err
        // Shadowing lets us reuse a variable name rather than forcing two unique variables
        // such as guess_str and guess, often used to convert values from one type to another

        /*
        Adding the keyword "mut" makes it mutable
        String::new is a function that returns a new instance of a String
        The :: syntax indicates that new is an associated function of the String type
        */

        /*
        Call stdin function from the io module to handle user input
        If module is not imported, the function is still usable by:
            std::io::stdin
        */

        println!("You guessed: {guess}");

        // Compare the two values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Add a break statement to make the program exit the loop
                println!("You win!");
                break;
            }
        }
    }
}

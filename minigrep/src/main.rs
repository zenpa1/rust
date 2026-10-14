use std::env;
use std::fs;

fn main() {
    /*
    std::env::args() - returns an Iterator
    > will panic! if any argument contains invalid Unicode (use std::env::args_os instead)

    .collect() - consumes the iterator and produces a collection
    > often requires type annotations as there are many aggregate data structures

    dbg!() - macro for diagnostics; writes to stderr instead of stdout unlike println!
    */
    let args: Vec<String> = env::args().collect();
    
    // Logic is now handed to parse_config, so main is not responsible for
    // mapping command line arguments and variables
    let (query, file_path) = parse_config(&args);

    // Takes file_path, opens file, and returns std::io::Result<String>
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path) // Return a tuple containing two string slices
}
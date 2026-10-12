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
    dbg!(&args);

    // &args[0] reserved for name of binary (used in calls and dynamic pathing)
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // Takes file_path, opens file, and returns std::io::Result<String>
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

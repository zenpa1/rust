use std::env;

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
}

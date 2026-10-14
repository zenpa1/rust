use std::env;
use std::fs;
use std::process;

fn main() {
    /*
    std::env::args() - returns an Iterator
    > will panic! if any argument contains invalid Unicode (use std::env::args_os instead)

    .collect() - consumes the iterator and produces a collection
    > often requires type annotations as there are many aggregate data structures

    dbg!() - macro for diagnostics; writes to stderr instead of stdout unlike println!
    */
    let args: Vec<String> = env::args().collect();

    // Instead of a separate function, we made a new function as an implementation of Config
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Takes file_path, opens file, and returns std::io::Result<String>
    // Also now uses config. to access
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// Introducing a struct to convey grouping and allow future maintainers to understand
// how the different values relate to each other and what their purpose is
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Changed to build as new is expected to never fail
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // Checks if the slice is long enough before accessing
        if args.len() < 3 {
            // // More appropriate for a programming issue than a usage issue
            // panic!("not enough arguments");

            // Return either an Err or an Ok() result instead
            // Error values will be string literals with a 'static lifetime
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone(); // .clone() - easy yet inefficient; avoids lifetime management
//     let file_path = args[2].clone();

//     Config { query, file_path } // Return a Config with two Strings
//     // Now also conveys that query and file_path are related
// }

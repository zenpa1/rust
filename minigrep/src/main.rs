use minigrep::search;
use minigrep::search_case_insensitive;
use std::env;
use std::error::Error;
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
    // let args: Vec<String> = env::args().collect();

    // Instead of a separate function, we made a new function as an implementation of Config
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // New: Change the build function to take ownership of an iterator as its argument instead
    // of borrowing a slice (env::args returns an iterator, so we instead pass ownership of the iterator
    // rather than collect iterator values into a vector and pass them to a slice)
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // run returns () if successful so we only care about detecting an error
    // meaning we don't need unwrap_or_else
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// Introducing a struct to convey grouping and allow future maintainers to understand
// how the different values relate to each other and what their purpose is
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// For now, Box<dyn Error> means the function will return a type that implements
// the Error trait, but no particular type, allowing us to return error values
// that may be of different types in different error cases

// dyn is short for dynamic
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Takes file_path, opens file, and returns std::io::Result<String>
    let contents = fs::read_to_string(config.file_path)?; // ? returns error value for caller to handle

    // Environment variable for the function to decide which function to use
    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(()) // Previously returned unit type, but now kept as a value inside Ok case
}

impl Config {
    // Changed to build as new is expected to never fail
    // fn build(args: &[String]) -> Result<Config, &'static str> {

    // New: signature utilizing a generic type with trait bounds
    // Additionally, env::args returns std::env::Args, which implements the Iterator trait
    // and returns String values
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // // Checks if the slice is long enough before accessing
        // if args.len() < 3 {
        //     // // More appropriate for a programming issue than a usage issue
        //     // panic!("not enough arguments");

        //     // Return either an Err or an Ok() result instead
        //     // Error values will be string literals with a 'static lifetime
        //     return Err("not enough arguments");
        // }

        // let query: String = args[1].clone();
        // let file_path: String = args[2].clone();

        // // env::var returns a Result that is Ok variant or Err variant depending if an environment variable is set
        // // is_ok() returns True if the result is Ok (we don't care about what value it is as long as it is set)
        // let env_ignore: bool = env::var("IGNORE_CASE").is_ok();

        // // CLI flag argument
        // let cli_ignore: bool =
        //     args.contains(&String::from("-i")) || args.contains(&String::from("--ignore-case"));

        // // Combine to compute final ignore_case boolean
        // // But what if I placed IGNORE_CASE=1 and --case-sensitive?
        // let ignore_case: bool = env_ignore || cli_ignore;

        // Explicit precedence hierarchy
        // 1. Manual case sensitivity for a single command while IGNORE_CASE=1
        // 2. -i or --ignore-case argument flag
        // 3. The environment variable gets the least priority as it is set for the whole session
        // let ignore_case: bool = if args.contains(&String::from("--case-sensitive")) {
        //     false
        // } else if args.contains(&String::from("-i"))
        //     || args.contains(&String::from("--ignore-case"))
        // {
        //     true
        // } else {
        //     env::var("IGNORE_CASE").is_ok()
        // };

        // Ok(Config {
        //     query,
        //     file_path,
        //     ignore_case,
        // })

        // New: Iterator revamp
        args.next(); // First value is the name of the program, so ignore it via next and do nothing

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

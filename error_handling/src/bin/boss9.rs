use std::fs;
use std::io::Error;
use std::num::ParseIntError;
use crate::game::Minion;

#[derive(Debug)]
enum LoadingError {
    // I/O Error (e.g., missing file)
    FileAccessFailed(Error),

    // Number Parse Error (e.g., "hi")
    IntegerParseFailed(ParseIntError),
}

impl From<Error> for LoadingError {
    fn from(err: Error) -> Self {
        LoadingError::FileAccessFailed(err)
    }
}

impl From<ParseIntError> for LoadingError {
    fn from(err: ParseIntError) -> Self {
        LoadingError::IntegerParseFailed(err)
    }
}

mod game {
    pub struct Minion {
        mana: u32,
        level: u32,
    }

    impl Minion {
        pub fn new(mana: u32, level: u32) -> Self {
            if mana < 1 || mana > 1000 {
                panic!("Minion's mana must be between 1 and 1000, got {}", mana);
            }

            if level < 1 || level > 10 {
                panic!("Minion's level must be between 1 and 10, got {}", level);
            }

            Minion { mana, level }
        }

        pub fn get_mana(&self) -> u32 {
            self.mana
        }

        pub fn get_level(&self) -> u32 {
            self.level
        }
    }
}

fn main() {
    match read_minion_data() {
        Ok(minion) => {
            println!("Success! The minion has {} mana and is Level {}.", minion.get_mana(), minion.get_level());
        }
        Err(LoadingError::FileAccessFailed(err)) => {
            println!("Failed to access file! {}", err);
        }
        Err(LoadingError::IntegerParseFailed(err)) => {
            println!("Failed to parse file contents! {}", err);
        }
    }
}

fn read_minion_data() -> Result<Minion, LoadingError> {
    // Read to string to skip byte management
    let string = fs::read_to_string("minion.txt")?;
    
    // Split by delimiter
    let mut parts = string.split(',');

    /*
    next() -> iterate one element
    expect() -> unwrap Option, panic! if invalid
    trim() -> remove invisible whitespace/newline characters
    parse::<u32>() -> parse to u32 value explicitly
    ? -> unwrap Result, propagate Err if invalid
    */
    let mana: u32 = parts.next().expect("Failed to parse mana.").trim().parse::<u32>()?;
    let level: u32 = parts.next().expect("Failed to parse level.").trim().parse::<u32>()?;

    // Return new Minion if successfully parsed
    // Not sure how to explain why it needs Ok(), but I assume it's because
    // it is part of a Result<T, E>, and Ok() and Err() are their respective wrappers?
    Ok(Minion::new(mana, level))
}

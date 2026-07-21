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
        LoadingGame::FileAccessFailed(err)
    }
}

impl From<ParseIntError> for LoadingError {
    fn from(err: Error) -> Self {
        LoadingGame::IntegerParseFailed(err)
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
    }
}

fn main() {
    match read_minion_data() {
        Ok(string) => {
            println!("Success!");
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
    fs::read_to_string("minion.txt")
    
    // Split by delimiter
    let parts = string.split(',');
    let mana: u32 = parts.next().parse::<u32>()?;
    let level: u32 = parts.next().parse::<u32>()?;

}

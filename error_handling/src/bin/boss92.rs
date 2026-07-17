use std::fs;
use std::io::Error;
use std::num::ParseIntError;

#[derive(Debug)]
enum SaveStateError {
    // I/O Error
    FileAccessFailed(Error),

    // Integer Parsing Error
    IntegerParseFailed(ParseIntError),
}

impl From<Error> for SaveStateError {
    // from std::io::Error -> SaveStateError::FileAccessFailed
    fn from(err: Error) -> Self {
        SaveStateError::FileAccessFailed(err)
    }
}

impl From<ParseIntError> for SaveStateError {
    // from std::num::ParseIntError -> SaveStateError::IntegerParseFailed
    fn from(err: ParseIntError) -> Self {
        SaveStateError::IntegerParseFailed(err)
    }
}

fn main() {
    match read_hitpoints_from_file() {
        Ok(hp) => println!("Success! HP is: {}", hp),
        // Wrap SaveStateError in Err() for matching Result statement
        Err(SaveStateError::FileAccessFailed(err)) => {
            println!("{err:?}");
            println!("No save file found. Starting a new journey.")
        }
        Err(SaveStateError::IntegerParseFailed(err)) => { 
            println!("{err:?}");
            println!("Save file corrupted. Please contact an admin.")
        }
    }
}

fn read_hitpoints_from_file() -> Result<u32, SaveStateError> {
    // Read Hit Point data to a u32
    // Expects the file at the ROOT of the directory (error_handling)
    let contents = fs::read_to_string("soulcrystal.txt")?;

    /*
    trim() returns a string slice with trailing and leading whitespaces removed.
    parse::<T> parses to a specific Type (turbofish syntax)
    ? automatically converts to whichever is expected from fn signature
    Ok() wrapping because parse returned a u32 value
    */
    Ok(contents.trim().parse::<u32>()?)
}
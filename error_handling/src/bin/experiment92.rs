use std::fs;
use std::fs::File;
// use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };

    // let greeting_file = File::open("hello.txt").unwrap(); // returns Ok value, panic! on Err

    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");
    match read_username_from_file() {
        Ok(str) => println!("Success! Hello {str}!"),
        Err(err) => println!("Could not proceed: {err:?}"),
    }

    // does not work as main returns () - unit type or nothingness
    let greeting_file = File::open("hello.txt")?;
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e), // doesn't need return as it is the last expression
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// // Shorter version by chaining method calls
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new(); // buffer

//     /*
//     Open a file, and if Ok(), use the value for read_to_string().
//         If not, convert error to io::Error and return early.

//     Afterward, buffer it to a string, and if Ok(), proceed.
//         If not, convert error to io::Error and return early.
//     */
//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username) // return if Ok
// }

// // Even shorter using a new fn
// fn read_username_from_file() -> Result<String, io::Error> {
//     // Convenient function as it is a fairly common operation
//     fs::read_to_string("hello.txt")
// }

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    /*
    lines() - iterator over the lines of a string
    next() - advances iterator and returns next value (first line)
    chars() - iterator over the chars of a string slice
    last() - last element in an iterator of chars
    */
}

fn read_username_from_file() -> Option<String> {
    // ok() -> converts from Result to Option
    let mut username_file = File::open("hello.txt").ok()?;

    // ok_or() -> converts from Option to Result (supply missing error data manually)
    // let weapon = inventory.get_equipped_weapon().ok_or(CombatError::NoWeaponEquipped);

    let mut username = String::new();
    username_file.read_to_string(&mut username).ok()?;
    Some(username)
}

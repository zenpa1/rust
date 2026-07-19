use std::fs;
use std::num::ParseIntError;

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

fn main() {}

fn read_minion_data() {
    // Read to string to skip byte management
    let result: Result<String, std::io::Error> = fs::read_to_string("minion.txt")
    let parsed_result = match result {
        Ok(string) => {
            // Split by delimiter
            let parts = string.split(',');
            let mana: u32 = parts.next().parse::<u32>()?;
            let level: u32 = parts.next().parse::<u32>()?;
        }
        Err(error) => return Err(error),
    }

}

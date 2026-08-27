// pub fn greeting(name: &str) -> String {
//     format!("Hello {name}!")
// }

// Bug: The input is not found in the output anymore
pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        // The greeting itself may change, but we want to ensure that
        // it still contains the text of the input parameter
        let result = greeting("Carol");
        // Add a second parameter to clarify what the actual output was
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{result}`");
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    // pub fn new(value: i32) -> Guess {
    //     if value < 1 || value > 100 {
    //         panic!("Guess value must be between 1 and 100, got {value}.");
    //     }

    //     Guess { value }
    // }

    // // Bug: remove the > 100 condition
    // pub fn new(value: i32) -> Guess {
    //     if value < 1 {
    //         panic!("Guess value must be between 1 and 100, got {value}.");
    //     }

    //     Guess { value }
    // }

    // Optional: adding the expected parameter to the should_panic attribute
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 { // more precise explanation
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // #[should_panic]
    // fn greater_than_100() {
    //     // If Guess is greater than 100 and it panics, the test passes
    //     Guess::new(200);
    // }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
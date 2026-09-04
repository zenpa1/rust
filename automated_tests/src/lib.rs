pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

// // Bug: The input is not found in the output anymore
// pub fn greeting(name: &str) -> String {
//     format!("Hello!")
// }

pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
        } else if value > 100 {
            // more precise explanation
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }

    // // Bug: swap bodies of if statements
    // pub fn new(value: i32) -> Guess {
    //     if value < 1 {
    //         // panic did not contain this expected string
    //         panic!("Guess value must be less than or equal to 100, got {value}.");
    //     } else if value > 100 {
    //         // instead got this, which is not part of the expected parameter
    //         panic!("Guess value must be greater than or equal to 1, got {value}.");

    //         // it helps us determine if the test suceeded due to the panic! we wanted
    //         // or if it did another panic! that we are not aware of
    //     }

    //     Guess { value }
    // }
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
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

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

    // We can use Result<T, E> to return an Err instead of panicking
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

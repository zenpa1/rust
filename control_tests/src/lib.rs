fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*; // able ot use prints_and_returns_10

    // prints_and_returns_10 tests
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4); // does not print by default on test success
        // can print via --show-output flag
        assert_eq!(value, 10);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(value, 5);
    // }

    // add_two tests
    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        let result = add_two(200);
        assert_eq!(result, 202);
    }
}

mod ultimate {
    use super::*;

    #[test]
    fn add_two_and_sixty_seven() {
        let result = add_two(67);
        assert_eq!(result, 69);
    }

    #[test]
    fn test_the_logger() {
        assert_eq!(true, true);
    }

        #[test]
    fn test_the_database() {
        assert_eq!(true, true);
    }

        #[test]
    fn test_logger_and_database() {
        assert_eq!(true, true);
    }
}
use std::fs;
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    // Trial A: Race condition in parallel read/write
    // Test X
    #[test]
    fn test_x() -> Result<(), std::io::Error> {
        // Write 100 to currently existing file
        let _ = fs::write("server_gold.txt", "100");

        // Read file
        let contents = fs::read_to_string("server_gold.txt")?;

        // Assert 100
        assert_eq!(contents.to_string(), "100");
        Ok(())
    }

    // Test Y
    #[test]
    fn test_y() -> Result<(), std::io::Error> {
        // Write 200 to currently existing file
        let _ = fs::write("server_gold.txt", "200");

        // Read file
        let contents = fs::read_to_string("server_gold.txt")?;

        // Assert 200
        assert_eq!(contents.to_string(), "200");
        Ok(())
    }

    // Trial B: On test success, also show the println! statement
    #[test]
    fn show_ascii_map() {
        let result: u32 = 2 + 2;
        assert_eq!(result, 4);

        println!("Hi! I am an ASCII map (idk what to put here)");
    }

    // Trial C: A time-consuming task
    #[test]
    #[ignore]
    fn take_long() {
        thread::sleep(Duration::from_secs(4));
    }
}

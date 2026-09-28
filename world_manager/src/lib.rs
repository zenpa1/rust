use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    // Trial A
    // Test X
    #[test]
    fn test_x() -> Result<(), std::io::Error> {
        // Write 100 to currently existing file
        fs::write("server_gold.txt", "100");

        // Read file
        let contents = fs::read_to_string("server_gold.txt")?;

        // Assert 100
        assert_eq!(contents.to_string(), "100");
        Ok(())
    }

    // Test Y
    fn test_y() {
        // Open a file

        
        // Write 200


        // Read file


        // Assert 200
    }

}

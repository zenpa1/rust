pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Mutable vector for storing matching lines
    let mut results: Vec<&str> = Vec::new();

    // Returns an iterator (line-by-line iteration of strings)
    for line in contents.lines() {
        // Check whether current line contains query string
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Failing tests before making them pass maintains high test coverage
    fn one_result() {
        let query = "duct"; // for productive

        // Apparently, indents inside here create whitespaces
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

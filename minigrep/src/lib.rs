// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     // Mutable vector for storing matching lines
//     let mut results: Vec<&str> = Vec::new();

//     // Returns an iterator (line-by-line iteration of strings)
//     for line in contents.lines() {
//         // Check whether current line contains query string
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }

// New: Concise way using iterator adapter methods, functional programming style prefers
// to minimize the amount of mutable state to make code clearer
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Mutable vector for storing matching lines
    let mut results: Vec<&str> = Vec::new();

    // Lowercase to make insensitive (via shadowing)
    // Returns a String because changing character casing can alter byte representation
    // or byte count in UTF-8
    let query: String = query.to_lowercase();

    // Returns an iterator (line-by-line iteration of strings)
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
    fn case_sensitive() {
        let query = "duct"; // for productive

        // Apparently, indents inside here create whitespaces
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

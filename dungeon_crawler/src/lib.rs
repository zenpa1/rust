use std::error::Error;
use std::fs;

pub struct Config {
    pub hero_name: String,
    pub sector_id: String,
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Checks if the slice is long enough before accessing
        if args.len() < 4 {
            return Err("Not enough arguments. Usage: cargo run <hero_name> <sector_id> <query>");
        } else if args.len() > 4 {
            eprintln!("Warning: Ignoring surplus arguments.");
        }

        // .clone() is good for now
        let hero_name: String = args[1].clone();
        let sector_id: String = args[2].clone();
        let query: String = args[3].clone();

        // Invoke via type namespace Self:: as it adapts if the struct name changes
        let file_path: String = Self::build_file_path(&sector_id);

        Ok(Config {
            hero_name,
            sector_id,
            query,
            file_path,
        })
    }

    fn build_file_path(sector_id: &str) -> String {
        format!("{}.txt", sector_id)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    // Deref coercion turns &String into &str automatically
    let matched: Vec<&str> = search(&config.query, &contents);

    println!(
        "Search initialized by {} in {}",
        config.hero_name, config.sector_id
    );

    println!("\n====== MATCHED ======");

    for line in &matched {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Create a mutable vector to keep matched lines
    // idiomatic over vec![] for creating empty vectors in generic code
    // whereas vec![] is often used when initializing with elements
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        // Check if line contains query
        if line.contains(query) {
            // Push if matched
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn search_one_result() {
        let query: &str = "sword";

        let contents: &str = "\
I am the bone of my sword.
Steel is my body and fire is my blood.
I have forged over a thousand blades,
Unknown to Death, nor known to Life.
I have withstood pain to create many weapons,
Yet those hands will never hold anything.
So as I pray, Unlimited Blade Works.
        ";

        assert_eq!(vec!["I am the bone of my sword."], search(query, contents));
    }
}

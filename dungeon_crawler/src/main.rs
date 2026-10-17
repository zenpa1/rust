use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

struct Config {
    hero_name: String,
    sector_id: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // Checks if the slice is long enough before accessing
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: cargo run <hero_name> <sector_id>");
        } else if args.len() > 3 {
            eprintln!("Warning: Ignoring surplus arguments.");
        }

        // .clone() is good for now
        let hero_name: String = args[1].clone();
        let sector_id: String = args[2].clone();

        // Invoke via type namespace Self:: as it adapts if the struct name changes
        let file_path: String = Self::build_file_path(&sector_id);

        Ok(Config {
            hero_name,
            sector_id,
            file_path,
        })
    }

    fn build_file_path(sector_id: &str) -> String {
        format!("{}.txt", sector_id)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    println!("\n====== PRINT ======\n{contents}");

    Ok(())
}

fn print_file(file_path: String) {
    let contents: String =
        fs::read_to_string(&file_path).expect("Should have been able to read the file");

    println!("\nPrinted {file_path}:\n{contents}")
}

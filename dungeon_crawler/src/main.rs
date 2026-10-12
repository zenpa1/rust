use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <hero_name> <sector_id>", args[0]);
        process::exit(1);
    }

    if args.len() > 3 {
        // If more than 3
        eprintln!("Warning: Ignoring surplus arguments.");
    }

    // Read and print file
    let hero_name: &str = &args[1];
    let sector_id: &str = &args[2];
    let file_path: String = create_file_path(sector_id);
    println!("Summoning hero '{hero_name}' into '{file_path}'...");
    print_file(file_path);
}

fn create_file_path(sector_id: &str) -> String {
    format!("{}.txt", sector_id)
}

fn print_file(file_path: String) {
    let contents: String =
        fs::read_to_string(&file_path).expect("Should have been able to read the file");

    println!("\nPrinted {file_path}:\n{contents}")
}

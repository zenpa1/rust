use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        // Read and print file
        let file_path: &str = &args[1];
        print_file(file_path);
        process::exit(1); // Prevents collecting arguments later
    } else if args.len() < 3 {
        // If less than 3
        eprintln!("Usage: {} <hero_name> <sector_id> <file_path>", args[0]);
        process::exit(1);
    }

    // Otherwise, proceed with the print_message fn
    let binary_name: &str = &args[0];
    let hero_name: &str = &args[1];
    let sector_id: &str = &args[2];

    if args.len() > 3 {
        // If more than 3
        eprintln!("Ignoring surplus arguments.");
    }

    print_message(binary_name, hero_name, sector_id);
}

fn print_message(bn: &str, hn: &str, si: &str) {
    println!("Using {bn} to find {hn} in {si}.");
}

fn print_file(file_path: &str) {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("\nPrinted {file_path}:\n{contents}")
}

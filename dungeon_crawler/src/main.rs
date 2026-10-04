use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        // If less than 3
        eprintln!("Usage: {} <hero_name> <sector_id>", args[0]);
        process::exit(1);
    }

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

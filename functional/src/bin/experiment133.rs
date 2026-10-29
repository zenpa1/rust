fn main() {
    // synthetic CLI args
    let synthetic_args: [String; 3] = [
        "experiment133".to_string(),
        "alice".to_string(),
        "INFO".to_string(),
    ];

    match parse_config(synthetic_args.into_iter()) {
        Ok((user, level)) => println!("Synthetic success: User={user}, Level={level}"),
        Err(e) => println!("Synthetic error: {e}"),
    }

    // real CLI args
    // cargo run -- bob WARN
    if std::env::args().len() > 1 {
        match parse_config(std::env::args()) {
            Ok((user, level)) => println!("CLI success: User={user}, Level={level}"),
            Err(e) => println!("CLI error: {e}"),
        }
    }

    // log filtering
    let sample_logs =
        "INFO: System started\nWARN: Low disk space\nINFO: User logged in\nERROR: Crash detected";
    let info_logs = filter_logs("INFO", sample_logs);
    println!("Filtered INFO logs: {info_logs:#?}");
}

fn parse_config(mut args: impl Iterator<Item = String>) -> Result<(String, String), &'static str> {
    args.next(); // skip binary name arg

    let target_user: String = match args.next() {
        Some(target_user) => target_user,
        None => return Err("No target user found."),
    };

    let log_level: String = match args.next() {
        Some(log_level) => log_level,
        None => return Err("No log level found."),
    };

    Ok((target_user, log_level))
}

fn filter_logs<'a>(keyword: &str, logs: &'a str) -> Vec<&'a str> {
    logs.lines()
        .filter(|line: &&str| line.contains(keyword))
        .collect()
}

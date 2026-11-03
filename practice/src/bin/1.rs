use std::io;

enum Type<'a> {
    Integer(i32),
    Float(f64),
    String(&'a str),
}

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // let result = input.trim().parse::<i32>();

    // match &result {
    //     Ok(val) => println!("i32: {val}"),
    //     Err(e) => println!("str: {}", input.trim()),
    // }

    // Add an enum + fn combo and you can do this:
    let result: Type = classify(&input);

    match result {
        Type::Integer(value) => println!("i32: {value}"),
        Type::Float(value) => println!("f64: {value}"),
        Type::String(value) => println!("String: {value}"),
    }
}

// fn classify(input: &str) -> Type<'_> {
// '_ is the placeholder lifetime which tells Rust to infer the appropriate lifetime rather
// than explicitly naming it

fn classify<'a>(input: &'a str) -> Type<'a> {
    // Classifies the input and returns the Type (enum)
    if let Ok(val) = input.trim().parse::<i32>() {
        println!("val is i32");
        Type::Integer(val)
    } else if let Ok(val) = input.trim().parse::<f64>() {
        println!("val is f64");
        Type::Float(val)
    } else {
        println!("val is String");
        // to_string() creates a new owned String from a &str
        // Type::String(input.to_string())

        // Lifetime basically states how long a reference is guaranteed to remain valid
        // The function may end but the returned reference an remain valid long after
        // If a reference could outlive its referenced data, the program is rejected
        Type::String(input.trim())
    }
}

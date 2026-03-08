fn main() {
    println!("Hello, world!");

    another_function();
    adder_function(2, 8, 'C');
    // f(0); // Does not compile as fn must declare types of its parameters
    f(0);

    let x = five();
    println!("This value is x: {x}");

    let y = plus_one(5);
    println!("The value of y is: {y}");

    println!(
        "{}",
        plus_one({
            let y = 1;
            y + 1
        })
    )
}

fn another_function() {
    println!("Another function.");
}

// Function with parameters
fn adder_function(a: i32, b: i32, unit_label: char) {
    // One main string consisting of placeholders
    println!("{} {}", a + b, unit_label);
}

fn f(x: i32) {
    println!("{x}");
}

// Return values to the code that calls them
// Function return type is specified as -> i32
fn five() -> i32 {
    5
}
// Same as let x = 5;

fn plus_one(x: i32) -> i32 {
    x + 1
}

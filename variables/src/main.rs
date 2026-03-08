const TWO: u32 = 1 + 1; // Can be defined outside of a fn (global)

fn main() {
    // let x: i32 = 5; // immutable, therefore compiler error
    let mut x: i32 = 25; // Can only be defined inside a fn (local)
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("Two is {TWO}!");

    let y: i32 = 5; // First, bind x to 5 (5)
    let y: i32 = y + 1; // Take orig. value and add 1 (6)

    {
        let y = y * 2;
        println!("This inner scope's y is: {y}"); // Multiply y by 2 
    } // Makes an inner scope

    let mut num: u32 = 1;
    {
        let mut num: u32 = num;
        num += 2;
    }
    println!("num: {num}");

    println!("The value of y is: {x}");
}

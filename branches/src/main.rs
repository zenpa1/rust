fn main() {
    // let number = 3;
    // let number = 7;
    let number = 5;

    // Condition MUST be a bool
    if number < 5 {
        println!("condition was TRUE as {number}");
    } else if number > 5 {
        println!("condition was FALSE as {number}");
    } else {
        println!("num is 5");
    }
    // Too many else ifs can clutter code
    // More than one requires refactoring, use match instead

    // This will not work unlike JavaScript
    // if number {
    //     println!("num was three");
    // }

    // if is an expression, which can be used on the right side of a let variable
    let condition: bool = true;
    let ending: &str = if condition {
        "Good ending"
    } else {
        "Bad ending"
    };

    println!("{ending}");
}

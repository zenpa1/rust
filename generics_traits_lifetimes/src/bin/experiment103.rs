fn main() {
    // let r: &i32; // can be initialized without a value though!

    // {
    //     let x = 5;
    //     r = &x; // x does not live long enough
    // } // x is dropped here while it is still borrowed
    // in essence, the value that r refers to has gone out of scope
    // before we try to use it, leading to a compilation error!

    // this block can be commented out and it would still compile
    // because r was not used, so even if it doesn't have a value, it compiles
    // let x = 5;
    // r = &x;

    let x = 5;
    let r = &x;
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
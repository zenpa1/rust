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

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {result}");

    // references having different concrete lifetimes
    let string1 = String::from("long string is long"); // initially 'a

    {
        let string2 = String::from("xyz"); // the new 'a (smaller)
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}"); // valid as it is still within scope
    }
}

//     // scope outlive example
//     let string1 = String::from("long string is long"); // initially 'a
//     let result;

//     {
//         let string2 = String::from("xyz"); // the new 'a (smaller)
//         result = longest(string1.as_str(), string2.as_str());
//     } // string 2 dropped here while borrowed
//     println!("the longest string is {result}"); // borrow used here
//     // but string2 does not live long enough
// }

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// // with lifetime annotation
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

// what if we just returned x and never y?
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}
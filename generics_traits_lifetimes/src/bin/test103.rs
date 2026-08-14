// // Experiment 1
// fn main() {
//     let outer = String::from("hello");
//     let inner;
//     {
//         // let inner = "world!"; // string literal
//         // // hardcoded directly into the binary of our program
//         inner = "world";
//     }

//     let result = longest(&outer, &inner); // inner not found in scope
//     println!("{result}");
// }

fn longest<'a>(outer: &'a str, inner: &'a str) -> &'a str {
    if outer.len() > inner.len() {
        outer
    } else {
        inner
    }
}

// // Experiment 2
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {result}");
// }

// Experiment 3
fn main() {
    let string1 = String::from("hello!");
    let string2 = "world";
    let result;

    result = longest(&string1, &string2);
    drop(string2); // if string literal, does nothing!
    // but if string, it is dropped as it is owned

    println!("{result}");
}
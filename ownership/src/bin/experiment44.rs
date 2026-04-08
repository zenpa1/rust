// // slice type
// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear();
// }

// // returns a byte index value into the String parameter
// fn first_word(s: &String) -> usize {
//     // convert to bytes as a String is UTF-8 encoded,
//     // meaning its characters are variable-length,
//     // and we only want to deal with 1 byte chars
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         // if value is space (b' ')
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// // string slice; reference to part of a String
// fn main() {
//     let s = String::from("hello world");

//     // [starting_index..ending_index]
//     // starting_index is the first position in slice
//     // ending_index is one more than last position in slice
//     let hello: &str = &s[0..5];
//     let world: &str = &s[6..11]; // 11, not 10
//     let s2: &String = &s;

//     println!("{hello}");
//     println!("{world}");
//     println!("{s2}");
// }

// // range syntax
// fn main() {
//     let s = String::from("Astra Vindictus");

//     // index zero can drop value
//     let mut first_slice = &s[0..5];
//     println!("first slice: {}", first_slice);
//     first_slice = &s[..5];
//     println!("first slice after reassignment: {}", first_slice);

//     // last byte can drop value
//     let length = s.len();

//     let mut last_slice = &s[6..length];
//     println!("last slice: {}", last_slice);
//     last_slice = &s[6..];
//     println!("last slice after reassignment: {}", last_slice);

//     // drop both for entire string
//     let whole_slice = &s[..];
//     println!("whole slice: {}", whole_slice);
// }

// rewritten first_word function
fn main() {
    let string = String::from("hello world");
    let first = first_word(&string);
    let second = second_word(&string);
    println!("{} {}", first, second);

    // breaking changes
    // let mut s = String::from("hello world");
    // let word = first_word(&s); // immutable borrow
    // s.clear(); // mutable borrow
    // println!("the first word is {}", word); // immutable borrow used here
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // first index to space (exclusive)
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..]; // after space until end
        }
    }

    &s[..]
}

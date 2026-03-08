// fn main() {
//     // Tuple - fixed length, multiple types
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (x, y, z) = tup; // destructuring
//     println!("The value of y is {y}");

//     // direct tuple access
//     let five_hundred = tup.0;
//     let six_point_four = tup.1;
//     let one = tup.2;

//     // A tuple without values is a unit
//     // Expressions implicitly return the unit value IF they do not
//     // return any other value

//     let mut numbers: (i32, i32) = (1, 2);
//     numbers.0 = 0;
//     numbers.1 += 5;
//     let second_number = numbers.1;
//     println!("{second_number}");

//     // Array - fixed length, same type
//     let array: [i32; 5] = [1, 2, 3, 4, 5]; // type annotation + element count

//     // initialize an array with the same value for each element
//     let three_array: [i32; 5] = [3; 5];

//     // access by indexing
//     let first = array[0];
//     let second = array[1];
// }

// use std::io;

// fn main() {
//     // This program results in a runtime error upon invalid index

//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line.");

//     let index: usize = index.trim().parse().expect("Index entered is not valid.");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// fn main() {
//     /* Create a tuple containing two arrays
//     The first array has 2 elements of value "1"
//     The second array has 4 elements of value "3" */
//     let t = ([1; 2], [3; 4]);

//     // Destructure tuple into a, b
//     let (a, b) = t;

//     // Use placeholder technique with expression afterward
//     println!("{}", a[0] + t.1[0])
// }

fn main() {
    let message = "Hello, world!";
    let message_array = [message; 3];
    let element = message_array[2];
    println!("{element}");
}

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(&m1, &m2);
//     let s = format!("{} {}", m1, m2);
//     println!("{s}");
// }

// fn greet(g1: &String, g2: &String) {
//     println!("{} {}", g1, g2);
// }

// // mutating owner while reference still exists
// fn main() {
//     let mut m1 = String::from("Hello"); // mutable
//     let m2 = String::from("world");
//     let r1 = &m1; // set r1 as a reference to m1

//     /* r1 points to m1, which makes Rust freeze m1 AS LONG AS r1 is in use
//     if m1.push_str occurs, the heap allocation might grow and REALLOCATE
//     r1 would then point to a deallocated stack frame or heap address (dangling pointer)
//     */
//     m1.push_str("!!!"); // mutable borrow
//     println!("{}", r1); // but error: also borrowed as immutable here
// }

// // double write issue
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s; // first pointer with WRITE
//     let r2 = &mut s; // second pointer with WRITE

//     println!("{} {}", r1, r2); // error: first mutable borrow and then second mutable borrow
//     /* Rust can only have ONE active mutable alias
//     many readers OR one writer only

//     if we push to r1 it may move to a new heap address, making r2 point to garbage memory
//     and if written, use-after free error
//     */
// }

// // dangling reference
// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     // returns a reference to a String
//     let s = String::from("hello");
//     &s
// } // but s goes out of scope here, and is dropped

// fn main() {
//     let mut x = Box::new(1);
//     let a: i32 = *x; // *x reads heap value
//     *x += 1; // as x is mutable

//     println!("a:{}, x:{}", a, x);

//     let r1: &Box<i32> = &x; // r1 points to x on stack

//     // double indirection (resolve two pointers to reach terminal value)

//     /*
//     *x follows box to heap value
//     &(*x) takes a reference to that location on the heap (points but does not own)
//     */
//     let b: i32 = **r1; // two to get us to heap value (b = *r1 -> *x -> heap alloc (1))
//     println!("{b}");

//     let r2: &i32 = &*x; // r2 points to heap value directly (I assume it is a pointer pointing to allocated value?)
//     let c: i32 = *r2; // one dereference reads it
//     println!("{c}");
// }

// // dereferencing something w/o Copy trait
// fn main() {
//     // x points to a box in the heap. that box points to Hello in another part of the heap.
//     let x = Box::new(String::from("Hello"));
//     // let a = *x; error: cannot move out of a Box

//     /*
//     *x on a Box<i32> is fine as integers are Copy
//     but *x on a Box<String> is pulling ownership out of box into a new symbol a
//     */
//     // both these solutions can view
//     let a = &*x; // a points to heap alloc
//     let b = x.as_str();

//     // println!("{}", x);
//     println!("a is {}, x is still {}", a, x);
// }

// fn main() {
//     let mut str = String::from("hello");
//     let new_str = *str;
//     new_str.push_str(" world");

//     println!("{}", new_str);
// }

// fn main() {
//     let x = Box::new(0);
//     let y = Box::new(&x);
//     println!("{}", y);
//     println!("{}", *y);
//     println!("{}", **y);

//     // Everything above works due to Deref Coercion (implicit dereferencing)
//     // println! macro (and Display trait) uses Implicit Deref Coercion
//     // when passing y to println!, Rust looks at type
//     // if type does not directly implement Display, but CAN BE DEREFERENCED INTO something
//     // that does, Rust automatically follows the pointers until it finds the type that can print
//     // "ik u want the integer, lemme do it for you bb girl"
//     println!("{}", ***y);

//     let a = Box::new(50);
//     let b = Box::new(&a);

//     // let sum = y + 10; // error: does not implement Add integer
//     let sum = ***y + 10; // manual required for math

//     // implicit dereferencing usually happens during METHOD CALLS (usind . operator)
//     // and passed arguments

//     println!("{sum}");
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3]; // v points to [1, 2, 3]
//     let num: &i32 = &v[2]; // reference that points to 3
//     // v.push(4); // push an element, which reallocates to a new memory space if cap is maxed

//     // // The reason this does not work is because num does not point to the old v anymore
//     // println!("Third element is {}", *num);

//     // but if you change the order a bit:
//     println!("Third element is {}", *num);
//     v.push(4); // it works

//     // non-lexical lifetime (NLL) has the compiler seeing that num is never used again,
//     // effectively deleting it from the active symbol table, and the "Lock" on v is released
// }

// // vec of boxes
// fn main() {
//     // collection points to a vector (fat ptr) in the heap, and vec points to two separate heap allocations, 1 and 2
//     let mut collection = vec![Box::new(1), Box::new(2)];

//     // num points to vec's first box, which points to 1
//     let num = &*collection[0];

//     // presumably, this should break because if collection has to reallocate in order to
//     // fit the box pointing to 3, the reference would point to deallocated memory
//     collection.push(Box::new(3));

//     // in other words, immutable borrow of num collides with mutable borrow of .push()

//     println!("{num}");
// }

fn main() {
    let random_kid = String::from("Random Kid"); // random_kid points to Random Kid
    let mut king = String::from("Von");

    // we move random_kid to take_ownership frame, but since it doesn't return it, it drops
    let random_parent = take_ownership(random_kid); // so now we get a unit value instead
    // more specifically, it says take_ownership returned, but there's no data, so ()
    println!("{:?}", random_parent); // prints the received unit value

    borrow_read(&king);
    borrow_write(&mut king); // just needs a mutable reference
}

// Contract A: I need to OWN the string and may drop it
fn take_ownership(s: String) {
    println!("{}", s); // ok to print
} // drop s (goodbye data gg)

// Contract B: I need to READ the string, but will NOT CHANGE IT
fn borrow_read(s: &String) {
    println!("{}", s);
}

// Contract C: I need to WRITE (change) the string, need exclusive access
fn borrow_write(s: &mut String) {
    s.push_str("!");
    println!("{}", s);
}

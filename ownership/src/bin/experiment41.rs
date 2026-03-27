// fn main() {
//     // let a: Box<i32> = Box::new(15);
//     // let _b: Box<i32> = a;
//     // let _c: Box<i32> = Box::new(15);

//     // -- breaking changes --
//     // using value after move
//     // println!("{}", a); // value borrowed here after move
//     // Rust invalidates 'a' to prevent "double free" errors
//     // double free is when two variables try to deallocate the same memory

//     // scope dropping
//     // {
//     //     let b: Box<i32> = a; // 'a' moves into this inner scope
//     // } // drop 'b'
//     // println!("{}", a); // value borrowed here after move

//     // borrowing stack value vs heap value
//     // let x = 5; // x = 5
//     // let _y = x; // y = 5
//     // println!("{}", x); // integers have copy trait

//     // let a = Box::new(5); // point to 5 in heap
//     // let b = a; // let b point to 5 in heap, remove pointer from a
//     // println!("{}", a); // error: value borrowed after moving to b

//     // modifying heap value directly
//     // let a = Box::new(15);
//     // let mut b = a; // move a into mut b
//     // *b += 1; // access direct value of b in heap and increment
//     // println!("{}", b); // should work as b is mutable and dereference modifies heap value instead

//     // // shadowing
//     // let a = Box::new(15);
//     // let a = Box::new(36); // shadows by dropping first a immediately
//     // println!("{}", a);

//     // mutable first var
//     // let mut a = Box::new(10); // slot on stack named a, holding pointer to 10
//     // let b = a; // move pointer from slot a into slot b; a is now uninitializaed
//     // a = Box::new(30); // since a was declared mut, rust doesn't have to drop anything in a before putting a pointer to 30 there
//     // println!("a: {}. b: {}", a, b); // works??

//     // let mut a: Box<i32> = Box::new(10); // overwritten before being read
//     // a = Box::new(30); // a still owned 10 when assigned to 30, dropping 10 automatically
//     // // if slot is empty (after a move), assignment fills it
//     // // if slot is full, assignment drops old data THEN fills it
//     // println!("{}", a);
// }

// // collections use boxes section
// fn main() {
//     let first = String::from("Astrid"); // first points to Astrid in heap
//     let full = add_suffix(first); // move name to full, pointing to same heap alloc
//     // println!("{first}"); // error: value borrowed here after move (moved to name, then full)
//     println!("{full}");

//     // pointing to deallocated memory is NOT an issue, but using the pointer IS
// } // drop full

// fn add_suffix(mut name: String) -> String {
//     // move first to name (local), first in main frame becomes invalid

//     // push operation
//     // check if heap allocation has enough capacity
//     // it doesn't? reallocate to another memory space, push new string in, then deallocate
//     // old memory space allocation
//     name.push_str(", Knight of Valhalla");
//     name // return name as expression
// }

// // cloning to avoid moving data
// fn main() {
//     let first = String::from("Gwyndolin"); // first allocates and points to Gwyndolin
//     let first_clone = first.clone(); // first_clone allocates a new Gwyndolin and points to it
//     // now there are two instances of Gwyndolin in the heap
//     let full = add_suffix(first_clone); // move name to full

//     // // -- breaking changes --
//     // first.push_str(" the Old"); // first cannot be borrowed as mutable
//     let x: u32 = 5;
//     let result: u32 = add_integer(x);
//     println!("{}", result);

//     println!("{full}, originally {first}");
//     // at the point of printing, we have two allocations in the heap
//     // first -> Gwyndolin
//     // full -> Gwyndolin, Devourer of Gods
// }

// /*
// Keep in mind the "Transfer of Rights" for ownership
// If you own first_clone, you have an immutable handle to that data
// However, when you pass it to add_suffix(mut name: String), a MOVE occurs
// The moment you hand over the ownership, the function can decide what they want to do with it,
// making it mutable, and because first_clone was moved, there is no conflict in this

// The new owner (name) has full control over that memory and can declare it as mut in its
// local scope
// */
// fn add_suffix(mut name: String) -> String {
//     // move first_clone to name, removing first_clone's ptr
//     name.push_str(", Devourer of Gods");
//     name // move to whoever is the caller
// }

// fn add_integer(mut number: u32) -> u32 {
//     number += 5;
//     number
// }

// // shadowing variable
// fn main() {
//     // the name owns the rights, not the data
//     // so moving a into mutable a hands it down, giving ownership rights to mut a
//     let a = String::from("immutable"); // a points to immutable
//     let mut a = a; // move a into a new mutable a
//     a.push_str("...or is it?"); // reallocate to new memory
//     let a = a; // shadow once more into a new, immutable a
//     println!("{}", a);
// }

// // shadowing in nested scope
// fn main() {
//     let a = String::from("Original");
//     {
//         let a = a; // a moves into local a
//         println!("Inner: {a}"); // ok
//     } // drop local a

//     println!("Outer: {a}"); // error: value borrowed here after move
// }

// fn main() {
//     let _a_num: i32 = 4;
//     make_and_drop();
// }

// fn make_and_drop() {
//     let _a_box = Box::new(5);
// } // drops a_box

// -- breaking changes --
// fn main() {
//     let ptr = make_and_drop();
//     println!("{:?}", ptr);
// }
// // dangling reference error
// fn make_and_drop() {
//     let a_box = Box::new(5);
//     &a_box
// } // a_box drops here, so why bother keeping the reference?

// // scope is max lifetime but ownership can end sooner
// fn main() {
//     make_and_drop();
// }
// fn make_and_drop() {
//     let a_box = Box::new(5);
//     drop(a_box); // manual drop (in real situations, ownership can do this)

//     println!("{}", a_box); // error: value borrowed here after move
// }

fn main() {
    let a_num = 4;
    let b = make_and_drop(); // only legal way to save data
    println!("{}, {:?}", a_num, b);
}

fn make_and_drop() -> Box<i32> {
    let a_box = Box::new(5);
    // try to leak it without returning it
    // a_box // move ownership out to caller
    a_box; // error: expected Box, found () or nothing
    // basically means do a_box, then throw the result away (statement)

    // btw, zero functional difference between "return" keyword and using an expression
    // (variable name without a semicolon)
    // both move ownership to caller
    // expression is used more often, return is used for early returns (in if statement for ex.)
}

// fn main() {
//     println!("Hello world!");

//     let name = vec![String::from("Gwyn")];
//     let full_name = stringify_name_with_title(&name);
//     println!("{}", full_name);
// }

// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut full = name.join(" ");
//     full.push_str(", the Devourer of Gods");
//     full
// }

// fn main() {
//     // copy a number out of a vector

//     // initialize v as a vector with i32s
//     let v: Vec<i32> = vec![0, 1, 2];

//     // initialize n_ref as an i32 reference, pointing to v's first index
//     let n_ref: &i32 = &v[0];

//     // initialize n with the i32 value found by dereferencing n_ref
//     // dereferencing just expects the R permission
//     let n: i32 = *n_ref;
// }

// fn main() {
//     // name, name.0, and name.1 get RWO
//     let mut name: (String, String) = ( // tuple
//         String::from("Ferris"),
//         String::from("Rustacean")
//     );

//      // name and name.0 get R only
//      // first gets R and O, *first gets R only (borrows name.0)
//     let first: &String = &name.0;

//     /*
//     Why is name.1 unaffected? PARTIAL BORROWING
//     tuple or structs are made of completely separate, distinct physical slots in
//     memory

//     let first = &name.0 does the ff:
//     2. name is partially locked (loses O), cannot drop name as it destroys name.0
//     and it loses W (cannot overwrite whle tuple as it would overwrite name.0)
//     3. name.1 is completely ignored as it is a disjoint (separate path) from name.0.
//     Mutating name.1 poses zero threat to memory address that first is pointing to,
//     thereby keeping its W permission

//     apartment analogy:
//     name.0 = bedroom A
//     name.1 = bedroom B

//     when creating 'first', you are letting a guest stay in bedroom A
//     - cannot demolish apartment (name loses O)
//     - cannot remodel bedroom A (name.0 loses W)
//     - but guest does not care what happens to bedroom B (name.1 keeps W)
//     */
//     name.1.push_str(", Esq."); // reqs: R and W
//     println!("{first} {}", name.1);
// }

// fn main() {
//     let mut name = (
//         String::from("Ferris"),
//         String::from("Rustacean")
//     );

//     let first = get_first(&name); // removes all indexes W
//     // even name.0

//     name.1.push_str(", Esq.");
//     println!("{first} {}", name.1);
// }

// fn get_first(name: &(String, String)) -> &String {
//     &name.0
// }

// fn main() {
//     let mut a = [0, 1, 2, 3];
//     let x = &mut a[1];
//     // unlike tuples, there are no diff places for a[0], a[1], etc.
//     // uses a single place a[_] that represents all indexes of a

//     *x += 1;
//     // x and *x implicitly drop here

//     println!("{a:?}");
//     // :? is a formatting directive that tells Rust to use the
//     // std::fmt::Debug
// }

//     let v: Vec<String> = vec![String::from("Hello world")]; // declare a vector of strings

//     // String does not implement Copy trait, use a reference if need to read

//     let s_ref: &String = &v[0]; // declare String reference pointing to v's first index
//     println!("{s_ref}!"); // immutable reference, does not take ownership of string by deref
// }

// fn main() {
//     // -- DEEP COPY --
//     let v: Vec<String> = vec![String::from("Hello world")]; // declare a vector of strings

//     /*
//     Performing .clone() on a String performs a DEEP COPY
//     - Goes to OS, asks for a new chunk of heap memory, and physically copies every
//       single character from the original string into this new space
//     - There are now TWO "Hello world" strings on th heap, v owns one and s owns the other
//     */
//     let mut s: String = v[0].clone(); // clone first index of v, not sure if this allocates a new heap object

//     /*
//     Upon s.push('!'), Rust checks if s's specific heap buffer has enough capacity
//     - If not, it reallocates s to a larger heap space, which has ZERO EFFECT on v
//     */
//     s.push('!'); // String is a wrapper around a Vec<u8> (vector of bytes)

//     println!("{s}");
// }

// fn main() {
//     // -- TAKING OWERNSHIP --
//     let mut v: Vec<String> = vec![String::from("Hello world")]; // declare a vector of strings

//     /*
//     Unlike clone(), remove() does NOT allocate any new heap memory
//     - text sits in exact same physical location on heap

//     1. v.remove(0) extracts the "fat pointer" (String data structure on stack) out of vector
//     2. Shift any remaining elements in vector to left to fill gap (but rn it is empty)
//     3. Hands the pointer to s, making it the sole owner of the original heap data
//     4. Because s is declared as mutable, you are free to push to it
//     */
//     let mut s: String = v.remove(0); // move string out of vector?
//     s.push('!');
//     println!("{s}");
//     assert!(v.len() == 0);

//     /*
//     APPROACH 1: Borrow it &v[0], for reading (fastest, no memory alloc)
//     APPROACH 2: Clone it v[0].clone(), for mutating a copy w/o destroying orig vector (slowest, allocates new heap memory)
//     APPROACH 3: Remove it v.remove(0), mutate original string (fast, shifts vector ele., transfers ownership)
//     */
// }

fn main() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));

    let first = &name.0; // borrow name.0

    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}

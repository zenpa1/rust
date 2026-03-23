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

fn main() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    // unlike tuples, there are no diff places for a[0], a[1], etc.
    // uses a single place a[_] that represents all indexes of a

    *x += 1;
    // x and *x implicitly drop here

    println!("{a:?}");
    // :? is a formatting directive that tells Rust to use the
    // std::fmt::Debug
}
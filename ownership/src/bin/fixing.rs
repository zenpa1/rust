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

fn main() {
    // copy a number out of a vector

    // initialize v as a vector with i32s
    let v: Vec<i32> = vec![0, 1, 2];

    // initialize n_ref as an i32 reference, pointing to v's first index
    let n_ref: &i32 = &v[0];

    // initialize n with the i32 value found by dereferencing n_ref
    // dereferencing just expects the R permission
    let n: i32 = *n_ref;
}
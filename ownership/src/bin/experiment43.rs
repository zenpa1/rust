// // move ownership of string out of fn
// fn main() {
//     // let new_string = return_a_string();
//     let test_string = String::from("Adventurer");
//     println!("{}", return_a_string());
//     println!("{}", return_reference(&test_string));
// }

// fn return_a_string() -> String {
//     let s = String::from("Initializing lockdown sequence...");
//     s
// }

// // return a string literal (lives forever)
// fn return_a_string() -> &'static str {
//     "Astrid, Lacerta, and Vesper"
// }

// fn return_reference(original: &String) -> &str {
//     original
// }

// clone inut
fn main() {
    let mut name = vec![String::from("Iron")];
    // let first = &name[0];
    // let all = stringify_name_with_title(&name);
    println!("{}", add_final_word(&mut name));
}

// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut full = name.join(" Sword");
//     full
// }

// add second part
fn add_final_word(name: &mut Vec<String>) -> String {
    let word = String::from("Sword");
    name.push(word);
    let full = name.join(" ");
    full
}

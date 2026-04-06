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

// // clone inut
// fn main() {
//     let mut name = vec![String::from("Iron")];
//     // let first = &name[0];
//     // let all = stringify_name_with_title(&name);
//     println!("{}", add_final_word(&mut name));
// }

// // fn stringify_name_with_title(name: &Vec<String>) -> String {
// //     let mut full = name.join(" Sword");
// //     full
// // }

// // add second part
// fn add_final_word(name: &mut Vec<String>) -> String {
//     let word = String::from("Sword");
//     name.push(word);
//     let full = name.join(" ");
//     full
// }

// fn main() {
//     let player: String = String::from("Ava");
//     println!("{}", award_title(&player));
//     println!("Player: {player}");
// }

// fn award_title(name: &String) -> String {
//     // Our goal is not to change the original, but rather, make a new one
//     // that way, we can use the original as is
//     let mut name = name.clone();
//     name.push_str(", The Fallen Angel");
//     name
// }

// // RPG-esque showcase of various ways of using mut
// fn main() {
//     let mut inventory: Vec<String> = Vec::new();
//     let mut chest_loot: String = String::from("Iron Sword"); // mut for shadowing

//     // no mut
//     inventory.push(get_item(chest_loot)); // You own it now!
//     println!("Current inventory: {:?}", inventory);

//     // mut
//     chest_loot = String::from("Golden Sword"); // shadow
//     inventory.push(upgrade_item(chest_loot));
//     println!("Current inventory: {:?}", inventory);

//     // no mut: &ref
//     let vase = String::from("vase");
//     inspect_item(&vase);

//     // no mut: &mut ref
//     let mut ava_inventory: Vec<String> =
//         vec![String::from("Health Potion"), String::from("Iron Shield")];

//     println!("Ava's current inventory: {:?}", ava_inventory);

//     // I wasn't sure how to dynamically pop a reference out,
//     // so I ended up making a different variable instead
//     let ava_shield = ava_inventory.pop();

//     println!("Ava's current inventory after pop: {:?}", ava_inventory);

//     // What is Option here? Why does it display:
//     // Ava's popped item: Some("Iron Shield")
//     println!("Ava's popped item: {:?}", ava_shield);

//     // Alternative for now:
//     let mut ava_spear: String = String::from("Golden Spear"); // required mut apparently

//     // I made a mistake putting it in the println! macro but it never
//     // actually retuns a value so, keep it out!
//     upgrade_party_member_item(&mut ava_spear);
//     println!("Ava's current spear: {}", ava_spear);

//     // println!("{}", upgrade_party_member_item(&mut ava_shield));

//     // mut: &ref
//     let library: Vec<String> = vec![
//         String::from("Riddle me This!"),
//         String::from("Lost Souls"),
//         String::from("Adventuring 101"),
//     ];

//     // mut to look at new things
//     let mut eyes_pointed: &String = &library[0];
//     println!("Now looking at... {eyes_pointed}");

//     // // broken
//     // switch_target(&eyes_pointed, &library);
//     // print!("Switched view! Now looking at... {eyes_pointed}");

//     eyes_pointed = switch_target(&library);
//     println!("Switched view! Now looking at... {eyes_pointed}");
// }

// // no mut
// fn get_item(item: String) -> String {
//     // I'm not sure how to implement objects (Sword) yet, but I think
//     // that's the next chapter, so take it slow for now
//     println!("You have gained: {item}");
//     item
// }

// // mut
// fn upgrade_item(mut item: String) -> String {
//     item.push_str(" +1"); // You own it, you change it
//     item
// }

// // no mut: &ref
// fn inspect_item(item: &String) {
//     // read-only, so just use it for display functions
//     println!("Inspecting a {item}... you see nothing wrong with it.");
// }

// // no mut: &mut ref
// fn upgrade_party_member_item(item: &mut String) {
//     // does not own, but has the "key"
//     item.push_str(" +1");
//     // used for cases where we just need to modify the item,
//     // but not return it, as it owns to another member
// }

// // // mut: &ref (broken)
// // fn switch_target(mut target: &String, library: &Vec<String>) {
// //     // no changing of books, simply changing what we look at
// //     // useful for view systems
// //     target = &library[1];
// // }

// fn switch_target<'a>(library: &'a Vec<String>) -> &'a String {
//     // Lifetime <'a> tells the compiler:
//     // "The reference I return lives exactly as long as the library you gave me."
//     &library[1] // return new view
// }

// fn main() {
//     let v: Vec<i32> = vec![0, 1, 2];
//     let n_ref: &i32 = &v[0];
//     let n: i32 = *n_ref;
//     println!("{n}");
// }

// how do we safely access an element of non-copy type vectors?
fn main() {
    // 1. avoid taking ownership of string
    // use immutable reference
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}");

    // 2. clone data to get ownership of string
    // while leaving vector alone
    let w: Vec<String> = vec![String::from("World hello!")];
    let mut str: String = w[0].clone();
    str.push('!');
    println!("{str}");

    // 3. use Vec::remove to move the string out of vector
    let mut x: Vec<String> = vec![String::from("Astra")];
    let mut removed_string: String = x.remove(0);

    removed_string.push('!');
    println!("{removed_string}");
    assert!(x.len() == 0);
}

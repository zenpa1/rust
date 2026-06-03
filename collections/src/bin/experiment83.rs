use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrite

    // if the key exists, its value should remain
    // if it does not exist, insert the new key and value
    // or_insert which presumably means 1 or 0, and do something according to that
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(75);

    println!("{scores:?}");

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue Score: {}", score);

    // Iterate over each kv pair using a for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name: String = String::from("Favorite color");
    let field_value: String = String::from("Blue");

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value); // value moved here

    let potion_name: String = String::from("Everlasting Day");
    let potion_effect: String = String::from("Provides night vision for 24 hours.");

    let mut potion: HashMap<&String, &String> = HashMap::new();
    potion.insert(&potion_name, &potion_effect);

    println!(
        "Potion Name: {}\nPotion Effect: {}",
        potion_name, potion_effect
    );

    for (key, value) in &potion {
        println!("{key}: {value}");
    }

    let mut damage_log = HashMap::new();

    damage_log.insert(String::from("Firebomb"), 5);

    // return a mutable reference to the value (either existing one or the new 0)
    let count: &mut i32 = damage_log.entry(String::from("Fireball")).or_insert(0); // if none, put 0

    *count += 3; // add 3 to fireball's mutable i32 reference

    println!("{damage_log:?}");

    // // double lookup penalty if entry is not used
    // // naive Option way
    // if damage_log.get("Fireball").is_none() {
    //     // Lookup 1: Hash the string, traverse the memory buckets, find nothing

    //     damage_log.insert(String::from("Fireball"), 0);
    //     // Lookup 2: Hash the string AGAIN, traverse the buckets AGAIN, find the empty slot and insert
    // }

    // println!("{damage_log:?}");

    // counting how many times a word appears in a text
    let text = "hello world wonderful world";

    let mut string_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = string_map.entry(word).or_insert(0);
        *count += 1;
    }

    // remember that iterating over a hash map happens in a random order
    println!("{string_map:?}");
}

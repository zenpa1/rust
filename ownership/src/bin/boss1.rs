// // The Greedy Blacksmith (Ownership)
// fn main() {
//     let hero_weapon = String::from("Excalibur");

//     appraise_weapon(hero_weapon);

//     // borrow checker error
//     println!("The hero swings {}!", hero_weapon);
// }

// fn appraise_weapon(weapon: String) {
//     // hero_weapon -> weapon (move)
//     println!("The Blacksmith examines {}...", weapon);
// } // weapon is dropped here

// My Version
fn main() {
    let hero_weapon = String::from("Excalibur");

    appraise_weapon(&hero_weapon);

    println!("The hero swings {}", hero_weapon);
}

fn appraise_weapon(weapon: &String) {
    /*
    Instead of moving hero_weapon -> weapon, we can use
    a reference to the String pointer instead, avoiding
    the performance overhead of cloning
    */
    println!("The Blacksmith examines {}...", weapon);
} // pointer to hero_weapon is dropped here

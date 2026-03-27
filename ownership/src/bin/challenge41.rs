fn main() {
    let hero: String = String::from("Astrid"); // hero allocates and points to Astrid
    let shout: String = hero.clone(); // shout makes a clone and points to another Astrid'
    let hero: String = transform(hero); // shadow hero
    // transform(hero); // error: moves to fn and drops right after as it is not reassigned
    println!("Hero: {}", hero);
    println!("Shout: {}", shout);

    /* expected output:
    two distinct allocations:
    hero = "Astrid the Brave"
    shout = "Astrid"
    */
}

fn transform(mut s: String) -> String {
    s.push_str(" the Brave");
    s
}

// // dropping shout
// fn main() {
//     let hero = String::from("Astrid");
//     let shout = hero.clone();
//     drop(shout);
//     println!("{}", hero);
// }

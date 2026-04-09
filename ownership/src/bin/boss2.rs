// // The Alchemist's Dillema (Aliasing)
// fn main() {
//     let mut potion = String::from("Health Potion");

//     let inspector = &potion; // immutable borrow
//     let brewer = &mut potion; // mutable borrow

//     brewer.push_str(" (Upgraded)"); // mutable borrow used

//     // borrow checker error
//     println!("The Inspector recorded: {}", inspector); // immutable borrow used

//     /*
//     The reason this won't work is because when we create
//     'inspector', we refer to the heap allocation "Health
//     Potion", and when we try to push_str in brewer, a
//     mutable reference to the same allocation, we may end
//     up deallocating the old heap and reallocating it to a
//     new location to make space for the pushed string.

//     According to the Pointer Safety Principle, data should
//     NEVER be aliased AND mutated at the same time.

//     In other words, many readers OR one writer, but not both.
//     */
// }

// My Version
fn main() {
    let mut potion = String::from("Health Potion");

    /*
    One possible solution without cloning is to change the
    order of code around to ensure that we create a
    reference, use it immediately, and then drop it
    implicitly after, to prevent both existing at the
    same time
    */

    let brewer = &mut potion;
    brewer.push_str(" (Upgraded)");

    // brewer implicitly drops here (non-lexical lifetime)
    // allows us to make a new, immutable reference,
    // as the mutable reference is gone

    let inspector = &potion;
    println!("The Inspector recorded: {}", inspector);
}

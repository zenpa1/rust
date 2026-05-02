// TASK 3 (7.4 Scope): Bring the re-exported `Ingot` and `Sword` into scope
// here so the `main` function can use them without typing `crafting_engine::`.

use forge::crafting_engine::{Ingot, Sword};

fn main() {
    // TASK 4 (7.2 Privacy): Even if your paths are right, the compiler will
    // scream here. Go back up into the `crafting_engine` and add the `pub`
    // keyword exactly where it is needed to make this code compile.
    // Remember: Do NOT make `durability` public!

    let iron_ingot = Ingot::new("Iron");
    let mut hero_sword = Sword::forge("Iron Broadsword", iron_ingot);

    hero_sword.swing();

    println!("The forge is working perfectly!");
}

// // The Illusionist's Mirage (Dangling Pointer)
// fn main() {
//     let my_pet = summon_familiar();

//     // borrow checker error
//     println!("I have summoned a {}", my_pet);
// }

// fn summon_familiar() -> &String {
//     /*
//     familiar is declared in the stack frame of
//     summon_familiar, not main, meaning it will drop
//     as soon as the call for summon_familiar() ends.

//     Giving a reference does not move it out of the
//     function, so it ends up pointing to deallocated
//     memory.
//     */
//     let familiar = String::from("Spectral Wolf");
//     &familiar
// }

// My Version
fn main() {
    let my_pet = summon_familiar();
    println!("I have summoned a {}", my_pet);
}

fn summon_familiar() -> String {
    let familiar = String::from("Spectral Wolf");
    familiar

    /*
    The solution is to return the String instead, moving
    its ownership to my_pet in main. That way, it will
    not deallocate until its usage is done in main's frame,
    instead of summon_familiar's frame.
    */
}

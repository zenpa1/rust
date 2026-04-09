// The Runestone Cipher (Slices)
fn main() {
    let spellbook = String::from("Fireball IceLance Heal");

    // Your Task: Replace the `???` with a String Slice (&str)
    // that points perfectly to the word "Fireball".
    // let first_spell: &str = &spellbook[..8]; // manual way
    // println!("Casting {}!", first_spell);

    // but let's take it a step further, more dynamically.
    // by basing it off the first space
    println!("Casting {}!", find_first_spell(&spellbook));
}

fn find_first_spell(spellbook: &str) -> &str {
    // u8 refers to bytes
    // we convert to bytes for fixed byte length in ASCII chars.
    let bytes: &[u8] = spellbook.as_bytes();

    for (index, &letter) in bytes.iter().enumerate() {
        if letter == b' ' {
            // return slice until index, which is until space
            return &spellbook[..index];
        }
    }

    // if no space, return whole slice
    &spellbook[..]
}

// crate root (src/main.rs)

use crate::garden::vegetables::Asparagus;

pub mod garden; // include code it finds in src/garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}

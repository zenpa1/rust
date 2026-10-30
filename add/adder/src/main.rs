// use rand; // cannot use unless added to its own Cargo.toml file

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}

use rand::Rng;

fn main() {
    println!("Roll the gacha!");

    let mut rng = rand::rng();
    let gacha_roll: u32 = rng.random_range(1..=100); // 1 to 100
    println!("{gacha_roll}");
}

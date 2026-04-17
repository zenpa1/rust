// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// #[derive(Debug)]
// enum PlayerData {
//     Warrior(Player),
//     Mage(Player),
// }

// #[derive(Debug)]
// struct Player {
//     name: String,
//     level: u32,
// }

// impl Player {
//     fn new(name: String) -> Self {
//         Self { name, level: 1 }
//     }
// }

// fn main() {
//     let _home = IpAddr::V4(String::from("127.0.0.1"));

//     let _loopback = IpAddr::V6(String::from("::1"));

//     let my_player: Player = Player::new(String::from("Astra"));

//     let my_warrior = PlayerData::Warrior(my_player);

//     println!("{:?}", my_warrior);
// }

// practice
// 1. Defining a basic Enum for status effects
#[derive(Debug)]
enum StatusEffect {
    Poisoned,
    Burning,
    Normal,
}

// 2. Defining a method directly on the Enum!
impl StatusEffect {
    fn print_warning(&self) {
        // Without `match`, we can't easily write different logic for each variant.
        // But we CAN print the variant itself using the Debug {:?} formatter!
        println!("  -> System Alert: Entity is currently {:?}", self);
    }
}

// A struct to tie it all together
#[derive(Debug)]
struct Enemy {
    name: String,
    status: StatusEffect,
    loot_gold: Option<u32>, // The enemy might drop gold (Some), or nothing (None)
}

impl Enemy {
    // Associated function to spawn an enemy
    fn new(name: String, status: StatusEffect, loot_gold: Option<u32>) -> Self {
        Self {
            name,
            status,
            loot_gold,
        }
    }

    fn defeat(&self) {
        println!("You defeated the {}!", self.name);

        // Calling our Enum method
        self.status.print_warning();

        // 3. Handling Option<T> WITHOUT the match operator
        if self.loot_gold.is_some() {
            // .unwrap() forcibly pulls the value out of `Some`.
            // WARNING: If you call .unwrap() on a `None`, your program will crash!
            // That is why we MUST check .is_some() first.
            let gold = self.loot_gold.unwrap();
            println!("  -> Looted {} gold coins!", gold);
        } else {
            println!("  -> The enemy dropped absolutely nothing...");
        }
        println!("--------------------------------");
    }
}

fn main() {
    // Instantiating our custom enum variants
    let fire_status = StatusEffect::Burning;
    let normal_status = StatusEffect::Normal;

    // Instantiating the built-in Option variants
    let big_loot = Some(150);
    let no_loot: Option<u32> = None; // Remember to type-annotate None!

    // Creating our enemies
    let fire_elemental = Enemy::new(String::from("Fire Elemental"), fire_status, no_loot);
    let goblin_king = Enemy::new(String::from("Goblin King"), normal_status, big_loot);

    // Running the logic
    fire_elemental.defeat();
    goblin_king.defeat();
}

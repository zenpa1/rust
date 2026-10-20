struct Hero {
    hp: u32,
    mp: u32,
    status: Option<Status>,
    inventory: Vec<Item>,
}

struct Potion {
    name: String,
    effect: String,
}

struct Weapon {
    name: String,
    damage: u32,
}

struct Armor {
    name: String,
    defense: u32,
}

enum Status {
    Burning,
    Poisoned,
    Frozen,
}

enum Item {
    Potion(Potion),
    Weapon(Weapon),
    Armor(Armor),
}

fn main() {}

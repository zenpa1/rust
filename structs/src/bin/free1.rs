struct Player {
    name: String,
    level: u32,
    class: Class,
}

impl Player {
    fn new(name: String, class: Class) -> Self {
        Self {
            name,
            level: 1,
            class,
        }
    }

    fn greet(&self) {
        println!(
            "Hi! I'm {}! I am currently a Level {} {:?}!",
            self.name, self.level, self.class
        );
    }

    fn use_ability_damage(&self, ability: &AbilityDamage) {
        println!(
            "{} uses ability_damage {} and deals {} DMG!",
            self.name, ability.name, ability.damage
        );
    }
}

#[derive(Debug)]
enum Class {
    Warrior,
    Archer,
    Rogue,
    Mage,
    Priest,
}

struct AbilityDamage {
    name: String,
    damage: u32,
}

impl AbilityDamage {
    fn new(name: String, damage: u32) -> Self {
        Self { name, damage }
    }
}

fn main() {
    let new_player: Player = Player::new(String::from("Baldur"), Class::Warrior);
    new_player.greet();

    let smash: AbilityDamage = AbilityDamage::new(String::from("Smash"), 30);
    new_player.use_ability_damage(&smash);
}

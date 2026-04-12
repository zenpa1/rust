#[derive(Debug)]
struct Hero {
    name: String,
    level: u32,
    health: u32,
    max_health: u32,
}

impl Hero {
    // TODO: Recruiter (Associated Function)
    /*
    Write an associated function named `new` that acts as a constructor.
    It should accept a `name` of type String and return a new `Hero` instance.
    A fresh recruit always starts at level 1, with 100 health and 100 max_health.
    Bonus points if you use the field init shorthand!
    */

    // Creates a new Hero with default stats
    fn new(name: String) -> Self {
        Self {
            name,
            level: 1,
            health: 100,
            max_health: 100,
        }
    }

    // TODO: Skirmish (Method)
    /*
    Write a method named `take_damage`.
    It needs to mutate the hero's health, reducing it by an `amount` (u32).
    (Hint: Remember that health is a u32, so it cannot go below 0.
    You might want to look into the `.saturating_sub()` method, or just use basic if/else logic).
    */

    fn take_damage(&mut self, enemy_damage: u32) {
        // Clamps underflowing to 0 (ensures it cannot be negative value)
        // Change original hero_health to new value
        // &mut because we only want to change the health, not the whole Hero
        self.health = self.health.saturating_sub(enemy_damage)
    }

    // TODO: Tavern (Method)
    /*
    Write a method named `rest`.
    It should fully restore the hero's `health` back to their `max_health`.
    */

    fn rest(&mut self) {
        // Force hero_health to copy max_health
        self.health = self.max_health;
    }
}

fn main() {
    // TODO: Demonstration
    /*
    1. Instantiate a new Hero named "Aragon" using `Hero::new(...)`.
    2. Make them take 40 damage.
    3. Print their stats using println!("{:?}", your_hero);
    4. Make them rest.
    5. Print their stats again to prove they healed!
    */

    let mut your_hero: Hero = Hero::new(String::from("Aragon"));
    let enemy_damage = 40;
    println!("Your hero is created!\n {:?}", your_hero);
    your_hero.take_damage(enemy_damage);
    println!("Your hero takes {enemy_damage} damage!\n {:?}", your_hero);
    your_hero.rest();
    println!("Your hero takes a rest!\n {:?}", your_hero);
}

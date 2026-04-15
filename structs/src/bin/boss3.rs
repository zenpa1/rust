use std::cmp::min;

#[derive(Debug)]
struct Spell {
    name: String,
    mana_cost: u32,
}

#[derive(Debug)]
struct ManaCrystal {
    energy: u32,
}

#[derive(Debug)]
struct Wizard {
    name: String,
    max_mana: u32,
    current_mana: u32,
}

// create constructors for ease of use, instead of instantiating a struct instance manually
impl Spell {
    fn new(name: String, mana_cost: u32) -> Self {
        Self { name, mana_cost }
    }
}

impl ManaCrystal {
    fn new(energy: u32) -> Self {
        Self { energy }
    }
}

impl Wizard {
    // constructor
    fn new(name: String, max_mana: u32) -> Self {
        Self {
            name,
            max_mana,
            current_mana: max_mana, // i believe this will set current mana to max immediately?
        }
    }

    // fn cast
    // input: Spell, read-only (do not consume)
    // if enough mana, subtract mana_cost from current_mana and print success
    // if not enough mana, print fail
    fn cast(&mut self, spell: &Spell) {
        // &mut self to change the value without consuming the Wizard
        if self.current_mana >= spell.mana_cost {
            self.current_mana -= spell.mana_cost;
            println!("{} casted {}!", self.name, spell.name);
        } else {
            println!(
                "{} does not have enough mana to cast {}!",
                self.name, spell.name
            );
        }
    }

    // fn consume_crystal
    // input: ManaCrystal, consume
    // adds crystal energy to wizard current mana
    fn consume_crystal(&mut self, crystal: ManaCrystal) {
        // &mut self to change the value without consuming the Wizard
        // self.current_mana += crystal.energy; // supposed implementation

        // better implementation as it ensures it never goes beyond the max
        // logic: if crystal energy is smaller, than max mana, then give crystal energy value
        // but if max mana is smaller, then give max mana value
        // that way, it never exceeds

        // but there is a catch: ensure it's current mana + crsytal energy at the same time,
        // so that it does not add like 20 + 40 = 60 and all that
        self.current_mana += min(self.current_mana + crystal.energy, self.max_mana);
    }
}

fn main() {
    let mut my_wizard: Wizard = Wizard::new(String::from("Gandalf"), 50);
    let my_spell: Spell = Spell::new(String::from("Fireball"), 30);
    let my_crystal: ManaCrystal = ManaCrystal::new(40);

    println!("initial logs:");
    println!("my_wizard: {:?}", my_wizard);
    println!("my_spell: {:?}", my_spell);
    println!("my_crystal: {:?}", my_crystal);

    my_wizard.cast(&my_spell);
    println!("my_wizard after casting once: {:?}", my_wizard);

    my_wizard.cast(&my_spell);
    println!(
        "my_wizard after (attempting to) casting twice: {:?}",
        my_wizard
    );

    my_wizard.consume_crystal(my_crystal);
    println!("my_wizard after consuming crystal: {:?}", my_wizard);

    my_wizard.cast(&my_spell);
    println!("my_wizard after casting thrice: {:?}", my_wizard);

    // since we consumed my_crystal, it is not allowed to be used any further
    // trying to use it will be trying to access uninitialized or deallocated memory
    // println!("{:?}", my_crystal); // successfully consumed
}

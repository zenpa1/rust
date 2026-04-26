use std::u32;

enum MagicalEffect {
    Damage(u32),
    Heal(u32),
    Status(String),
}

struct Spell {
    name: String,
    // Option because some spells may have no effect
    effect: Option<MagicalEffect>,
}

impl Spell {
    // Spell constructor
    fn new(name: String, effect: Option<MagicalEffect>) -> Self {
        Self { name, effect }
    }

    fn cast(&self) {
        println!("Spell casted: {}", self.name);
        /*
        Oh my! I have never seen match &ref before! I assume this pattern
        can make it self, &self, or even &mut self?
        It was suggested as &self because... the String inside one of its
        parameters (effect) had MagicalEffect, with Status(String)??
        I'm so confused now...
        */
        match &self.effect {
            Some(MagicalEffect::Damage(dmg)) => println!("Dealt {} damage!", dmg),
            Some(MagicalEffect::Heal(heal)) => println!("Healed {} health!", heal),
            Some(MagicalEffect::Status(status)) => println!("Gained {} as status!", status),
            None => println!("It was a harmless light show..."),
        }
    }

    fn danger_check(&self) {
        // uhh idk how to explain this
        if let Some(MagicalEffect::Damage(dmg)) = &self.effect {
            println!(
                "DANGER: This '{}' spell can hurt people! Deals {} damage!",
                self.name, dmg
            );
        }
    }
}

fn main() {
    let offensive_spell: Spell =
        Spell::new("Fireball".to_string(), Some(MagicalEffect::Damage(30)));
    let healing_spell: Spell =
        Spell::new(String::from("Lesser Heal"), Some(MagicalEffect::Heal(15)));
    let harmless_spell: Spell = Spell::new("Fireworks".to_string(), None);

    offensive_spell.cast();
    offensive_spell.danger_check();
    healing_spell.cast();
    healing_spell.danger_check();
    harmless_spell.cast();
    harmless_spell.danger_check();
}

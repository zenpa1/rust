use grimoire::prelude::*;

fn main() {
    // Since we use grimoire::, we can skip straight to it
    let mut player_mana: ManaPool = ManaPool::new();

    let fire_spell: Fireball = Fireball { damage: 50 };

    fire_spell.cast(&mut player_mana); // Drain 30 mana
    fire_spell.cast(&mut player_mana); // Drain 30 mana
    fire_spell.cast(&mut player_mana); // Drain 30 mana

    let mana_potion: ManaPotion = ManaPotion { power: 50 };

    mana_potion.drink(&mut player_mana);

    let water_spell: Tsunami = Tsunami { damage: 30 };

    water_spell.cast(&mut player_mana);
}

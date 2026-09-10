// Test A Materials
pub fn heal(current_hp: u32, max_hp: u32, healing: u32) -> u32 {
    // Simple version of saturated addition
    if current_hp + healing > max_hp {
        max_hp
    } else {
        current_hp + healing
    }
}

// Test B Materials
enum Status {
    Burn(u32),
    Poison(u32),
}

struct Undead {
    hp: u32,
    resistance: Status,
    weakness: Status,
}

pub fn dmg(hp: u32, dmg: u32) -> u32 {
    hp - dmg // Unsigned integers can never be less than 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test A: Healing should never exceed maximum HP
    #[test]
    fn healing_limit() {
        let hp = 50;
        let max_hp = 100;
        let health_potion = 100;

        // Heal test
        let new_health = heal(hp, max_hp, health_potion);
        assert_eq!(new_health, 100); // Assert that it is the max value
    }

    // Test B: Undead is immune to poison
    #[test]
    fn poison_immunity() {
        let undead: Undead = Undead {
            hp: 50,
            resistance: Status::Poison(0), // Enums designed to hold a value (u32)
            weakness: Status::Burn(15),
        };

        match undead.resistance {}
    }
}

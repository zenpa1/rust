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
#[derive(Debug, PartialEq)]
enum Status {
    /*
    Concept - an enum wherein its variants are pure tags with no values attached,
    representing the IDEA such as Poison
    Instance - a struct or enum called ActiveDebuff that pairs an Element with a u32 value,
    such as a Poison debuff ticking for 10 damage
    */
    Burn,
    Poison,
}

// Test C Materials
pub fn damage(entity: &mut Undead, damage: u32) {
    // As we are dealing with u32 values, it will never be negative to begin with!
    // However, we need to account for an overkill anomaly, wherein it should not work
    // on a target with 0 HP left (useless calculation)

    if entity.hp == 0 {
        panic!("Entities with 0 HP must not be attacked.");
    } else {
        entity.hp -= damage; // Mutation, don't forget
    }
}

pub struct Undead {
    hp: u32,
    resistance: Vec<Status>, // A list of resistances
    weakness: Vec<Status>,   // A list of weaknesses
}

impl Undead {
    fn apply_status(&self, status: &Status) -> bool {
        /*
        matches! - for knowing something without extracting data, requires hardcoded reference,
        takes a variable and a pattern and returns a boolean, such as
        matches!(incoming_attack, Status::Poison) wherein Status::Poison is hardcoded

        if let - targets one specific variant and extracts its data in a single line,
        instead of full matching just for one variant, such as
        if let Status::Poison(10) = incoming_attack { ... damage_amount };
        */

        // Iterate through resistance list and match
        for resistance in &self.resistance {
            if status == resistance {
                // For comparing two variables dynamically
                println!("Undead is resistant to {status:?}!");
                return false;
            }
        }

        true
    }
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
            resistance: vec![Status::Poison],
            weakness: vec![Status::Burn],
        };

        // assert_eq!(undead.apply_status(&Status::Poison), true, "Alert: Poison was applied to an Undead entity."); // fail
        assert_eq!(
            undead.apply_status(&Status::Poison),
            false,
            "Alert: Poison was applied to an Undead entity."
        ); // pass
    }

    // Test C: panic! if an attack occurs on an entity with 0 HP
    #[test]
    #[should_panic(expected = "Entities with 0 HP must not be attacked.")]
    fn overkill_prevention() {
        let mut undead: Undead = Undead {
            hp: 50,
            resistance: vec![Status::Poison],
            weakness: vec![Status::Burn],
        };

        damage(&mut undead, 50); // Now 0
        damage(&mut undead, 50); // Should panic here
    }
}

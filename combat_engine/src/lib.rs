pub fn heal(current_hp: u32, max_hp: u32, healing: u32) -> u32 {
    // Simple version of saturated addition
    if current_hp + healing > max_hp {
        max_hp
    } else {
        current_hp + healing
    }
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
}

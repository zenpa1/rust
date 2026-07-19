mod game_types {
    #[derive(Debug)]
    pub struct HitPoints {
        value: u32,
    }

    impl HitPoints {
        // Take a raw u32, validate, and return the secure type
        pub fn new(value: u32) -> Self {
            // HP must be between 1 and 9,999
            if value < 1 || value > 9999 {
                // If a developer tries to hardcode or calculate an invalid HP,
                // the system aborts immediately as they broke the invariant contract
                panic!("HitPoints must be between 1 and 9999, got {}", value);
            }

            // If check passes, we lock the value inside the struct
            HitPoints { value }
        }

        // As 'value' is private, outside code CANNOT modify it directly
        // Use this method to read the number (getter)
        pub fn value(&self) -> u32 {
            self.value
        }
    }
}

fn main() {
    let mut player_hp: game_types::HitPoints = game_types::HitPoints::new(100);
    println!("Player has {} HP.", player_hp.value()); // getter

    // Instantly panics and crashes the program
    // let corrupted_hp = HitPoints::new(0);

    // player_hp.value = 5000;
    // println!("Player has {} HP.", player_hp.value()); // getter
    // // If the struct is in the same module, it still works
    // // due to visibility
}

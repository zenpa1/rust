use crate::game::AetherCore;

mod game {
    pub struct AetherCore {
        resonance: u32,
    }

    impl AetherCore {
        pub fn new(resonance: u32) -> Self {
            if resonance < 1 || resonance > 100 {
                panic!(
                    "Resonance value must be between 1 and 100, got {}.",
                    resonance
                );
            }

            AetherCore { resonance } // field init shorthand
            // Alternatively, if you intend to use value instead
            // AetherCore { resonance: value }
            // Why? Prevents unintended variable swapping
        }

        pub fn get_resonance(&self) -> u32 {
            self.resonance
        }
    }
}

fn main() {
    let mut core: AetherCore = AetherCore::new(42);
    println!("Your Aether Core's resonance is: {}", core.get_resonance())

    // core.resonance = 500;
}

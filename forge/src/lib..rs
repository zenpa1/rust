mod crafting_engine {
    mod materials {
        #[derive(Debug)]
        struct Ingot {
            metal_type: String,
            purity: u32,
        }

        impl Ingot {
            fn new(metal: &str) -> Self {
                Self {
                    metal_type: metal.to_string(),
                    purity: 100,
                }
            }
        }
    }

    mod weapons {
        // TASK 1 (7.3 Paths): The Sword struct needs to use the Ingot struct
        // from the sibling `materials` module. Fix this path using `super`.
        struct Sword {
            name: String,
            material: super::materials::Ingot,
            durability: u32, // Must remain PRIVATE
        }

        impl Sword {
            fn forge(name: &str, material: materials::Ingot) -> Self {
                Self {
                    name: name.to_string(),
                    material,
                    durability: 100,
                }
            }

            fn swing(&mut self) {
                self.durability -= 1;
                println!(
                    "Swung the {}! Durability is now {}.",
                    self.name, self.durability
                );
            }
        }
    }

    // TASK 2 (7.4 Re-exporting): The user shouldn't have to type
    // `crafting_engine::materials::Ingot`. Create a "Storefront" here at the
    // root of `crafting_engine`.
    // Write the `pub use` statements to re-export BOTH `Ingot` and `Sword`
    // directly from this level. Use a nested path `{}` to do it in as few lines as possible!
    pub use crafting_engine::{materials::Ingot, weapons::Sword};
}

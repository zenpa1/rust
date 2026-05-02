pub mod crafting_engine {
    pub mod materials {
        #[derive(Debug)]
        pub struct Ingot {
            metal_type: String,
            purity: u32,
        }

        impl Ingot {
            pub fn new(metal: &str) -> Self {
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
        pub struct Sword {
            name: String,
            material: super::materials::Ingot,
            durability: u32, // Must remain PRIVATE
        }

        impl Sword {
            pub fn forge(name: &str, material: super::materials::Ingot) -> Self {
                Self {
                    name: name.to_string(),
                    material,
                    durability: 100,
                }
            }

            pub fn swing(&mut self) {
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

    // already inside crafting_engine, so use self for relative path
    pub use self::{materials::Ingot, weapons::Sword};

    /*
    Why can't we just write pub use {materials::Ingot, weapons::Sword};?

    A use statement CANNOT start with an opening bracket
    self:: is the anchor point
    */

    // TASK 2: The Storefront using an absolute path (crate::)
    // pub use crate::crafting_engine::{materials::Ingot, weapons::Sword};
}

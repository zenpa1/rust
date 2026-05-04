pub mod mana; // when registering files, don't use crate::, use module name
pub use crate::mana::ManaPool;
pub mod spells;
pub use crate::spells::fire::Fireball;
pub use crate::spells::water::WaterSpout;
pub mod items;
pub use crate::items::potions::ManaPotion;

pub struct ManaPotion {
    pub power: u32,
}

impl ManaPotion {
    pub fn drink(&self, mana_pool: &mut crate::mana::ManaPool) {
        mana_pool.restore(self.power);
        println!("Drank a Mana Potion, restored {} mana!", self.power);
    }
}

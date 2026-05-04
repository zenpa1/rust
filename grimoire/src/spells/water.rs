pub struct WaterSpout {
    pub damage: u32,
}

impl WaterSpout {
    pub fn cast(&self, mana_pool: &mut crate::mana::ManaPool) {
        if mana_pool.drain(20) {
            println!("Casted Water Spout for {} damage!", self.damage);
        } else {
            println!("Not enough mana to cast Water Spout!");
        }
    }
}

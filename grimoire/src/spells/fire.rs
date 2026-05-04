// Fire Magic Sub-module
pub struct Fireball {
    pub damage: u32,
}

impl Fireball {
    pub fn cast(&self, mana_pool: &mut crate::mana::ManaPool) {
        if mana_pool.drain(30) {
            println!("Casted Fireball for {} damage!", self.damage);
        } else {
            println!("Not enough mana to cast Fireball!");
        }
    }
}

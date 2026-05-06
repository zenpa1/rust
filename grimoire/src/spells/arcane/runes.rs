pub struct TeleportRune {}

impl TeleportRune {
    pub fn activate(&self, mana_pool: &mut crate::mana::ManaPool) {
        if mana_pool.drain(99) {
            println!("Used Teleport Rune! It took 99 mana to use.");
        } else {
            println!("Could not use Teleport Rune (not enough mana).");
        }
    }
}

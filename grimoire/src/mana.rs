// Mana Module
// does not need "mod mana" as it is
// automatically namespaced
pub struct ManaPool {
    capacity: u32, // private
    current: u32,  // private
}

impl ManaPool {
    pub fn new() -> Self {
        Self {
            capacity: 100,
            current: 100,
        }
    }

    pub fn drain(&mut self, amount: u32) -> bool {
        // If there is enough mana, subtract and return true
        if self.current >= amount {
            self.current -= amount; // reduce current by amount
            return true;
        }

        return false;
    }

    pub fn restore(&mut self, amount: u32) {
        // Add potion amount to current mana
        let new_mana: u32 = self.current + amount;

        // Set current to whichever is SMALLER, new_mana or max capacity
        self.current = new_mana.min(self.capacity); // always 100
    }
}

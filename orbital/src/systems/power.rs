pub struct Reactor {
    core_temp: u32,
    available_energy: u32,
}

impl Reactor {
    pub fn boot_sequence() -> Self {
        Self {
            core_temp: 0,
            available_energy: 1000,
        }
    }

    pub fn draw_power(&mut self, amount: u32) -> bool {
        if self.available_energy >= amount {
            self.available_energy -= amount;
            self.core_temp += 10;
            return true;
        }

        // return false;
        // simplified
        false
    }
}
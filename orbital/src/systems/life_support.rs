pub struct OxygenGenerator {
    
}

impl OxygenGenerator {
    pub fn activate(&self, reactor: &mut super::power::Reactor) {
        if reactor.draw_power(150) {
            println!("O2 Flowing!");
        } else {
            println!("Critical Power failure!");
        }
    }
}
use orbital::prelude::*;

fn main() {
    let mut my_reactor = Reactor::boot_sequence();
    let my_oxygen_generator: O2Gen = O2Gen { };

    // O2Gen::activate(&my_oxygen_generator, &mut my_reactor);
    // cleaner way below
    my_oxygen_generator.activate(&mut my_reactor);
}

#[derive(Debug)]
struct MagicCore {
    element: String,
    power: u32,
}

#[derive(Debug)]
struct Amulet {
    owner: String,
    core: MagicCore,
}

impl Amulet {
    // standard constructor
    fn new(owner: String, element: String, power: u32) -> Self {
        Self {
            owner,
            core: MagicCore { element, power },
        }
    }

    // TODO: fix
    // This method takes ownership of a `new_core` and returns the completely updated `Amulet`.
    // It works, but it consumes ownership (`self`).
    fn absorb_core(&mut self, new_core: MagicCore) {
        self.core = new_core;

        // my fix: what if instead of returning an Amulet, we simply change the core, which just needs a mutable
        // reference, so that way, we don't unnecessarily consume the amulet in the main function
        // we are able to change the core without taking ownership this way
    }

    // This method tries to update the amulet while it sits on the anvil (&mut self).
    // Right now, it will NOT compile because it creates a hole on the anvil!
    // Fix this method using what you learned about `.clone()`.
    fn synchronize_core(&mut self, new_core: MagicCore) {
        // ERROR: cannot move out of `*self` which is behind a mutable reference
        self.absorb_core(new_core);

        // my fix: instead of trying to separate two separate functions, we simply just call the function
        // to absorb the new core, that way, we don't end up making a hole in the anvil
    }

    // TODO: sacrifice
    // Write a method named `transfer_power`.
    // It should borrow `&mut self` (the main amulet), and it must CONSUME a `sacrifice` Amulet.
    // It needs to add the sacrifice's `core.power` to the main amulet's `core.power`.
    // (You do not need to use absorb_core or synchronize_core for this; just mutate the power directly).
    fn transfer_power(&mut self, sacrifice: &Amulet) {
        self.core.power += sacrifice.core.power;

        // my implementation: add self.core.power with the sacrifice's core.power: mutates power directly
    }
}

fn main() {
    // create main amulet
    let mut my_amulet = Amulet::new(String::from("Aragon"), String::from("Fire"), 50);

    // create magic core
    let fresh_core = MagicCore {
        element: String::from("Lightning"),
        power: 100,
    };

    // synchronize new core
    my_amulet.synchronize_core(fresh_core);
    println!("After Sync: {:?}", my_amulet);

    // create sacrifice amulet
    let trash_amulet = Amulet::new(String::from("Goblin"), String::from("Mud"), 10);

    // transfer power
    // my_amulet.transfer_power(trash_amulet);
    my_amulet.transfer_power(&trash_amulet); // reference now
    println!("After Transfer: {:?}", my_amulet);

    // println!("Did the goblin survive? {:?}", trash_amulet);

    // my fix: the reason this did not work is it used to consume the trash amulet instead of using a reference to it
    // this can be intentional though, i'm simply showing how it would be fixed if it isn't supposed to consume
    println!("Did the goblin survive? {:?}", trash_amulet);
}

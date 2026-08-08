use std::fmt::Debug;

fn main() {
    let sword = Sword { physical_damage: 5 };

    let staff = Staff { magical_damage: 5 };

    duel(&sword, &staff);

    sword.inspect();
    staff.inspect();
}

pub trait Weapon {
    fn base_stat(&self) -> u32;

    fn weapon_rating(&self) -> u32 {
        self.base_stat() * 2 // required method feeds into default (this method)
        // now it uses the stat from the struct
    }

    fn weapon_name(&self) -> String;

    fn check_weapon_rating(&self) -> String {
        format!(
            "The combat rating of the {} is: {}",
            self.weapon_name(),
            self.weapon_rating()
        )
    }
}

pub trait Inspectable {
    // fn inspect(&self) {
    //     println!("Inspecting weapon...\n
    //     This is a {}", self.weapon_name()) // will not work as weapon_name is part of Weapon, so cannot be found in this scope
    // }

    fn inspect(&self);
}

impl<T: Weapon + Debug> Inspectable for T {
    fn inspect(&self) {
        println!(
            "\nInspecting weapon...\n
        This is a {}",
            self.weapon_name()
        ) // but we can here because T implements Weapon (and Debug)
        // unlocks weapons methods inside this specific block
    }
}

#[derive(Debug)]
pub struct Sword {
    physical_damage: u32,
}

impl Weapon for Sword {
    fn base_stat(&self) -> u32 {
        self.physical_damage
    }

    fn weapon_name(&self) -> String {
        String::from("Sword")
    }
}

#[derive(Debug)]
pub struct Staff {
    magical_damage: u32,
}

impl Weapon for Staff {
    fn base_stat(&self) -> u32 {
        self.magical_damage
    }

    fn weapon_rating(&self) -> u32 {
        self.base_stat() * 3 // different override, so diff implementation from sword
    }

    fn weapon_name(&self) -> String {
        String::from("Staff")
    }

    // override behavior
    fn check_weapon_rating(&self) -> String {
        format!(
            "The {} can deal {} magical damage.",
            self.weapon_name(),
            self.weapon_rating()
        )
    }
}

pub fn duel<T, U>(primary: &T, secondary: &U)
where
    T: Weapon + Debug,
    U: Weapon + Debug, // because a Sword doesn't need to be a Staff, yet both impl same traits
{
    // // compare which weapon rating is higher
    // if primary.weapon_rating() > secondary.weapon_rating() {
    //     println!(
    //         "A duel has begun!\n
    // Contestant 1: {primary:?}\n
    // Contestant 2: {secondary:?}\n
    // The winner is {} with a weapon rating of {}.",
    //         primary.weapon_name(),
    //         primary.weapon_rating(),
    //     );
    // } else {
    //     println!(
    //         "A duel has begun!\n
    // Contestant 1: {primary:?} with multipler of 2\n
    // Contestant 2: {secondary:?} with multiplier of 3\n
    // The winner is {} with a weapon rating of {}.",
    //         secondary.weapon_name(),
    //         secondary.weapon_rating(),
    //     );
    // };

    // meta method - extract a tuple of data INSTEAD OF THE STRUCT ITSELF
    let (winner_name, winner_rating) = if primary.weapon_rating() > secondary.weapon_rating() {
        (primary.weapon_name(), primary.weapon_rating())
    } else {
        (secondary.weapon_name(), secondary.weapon_rating())
    };

    println!(
        "A duel has begun!\n
    Contestant 1: {primary:?} with multipler of 2\n
    Contestant 2: {secondary:?} with multiplier of 3\n
    The winner is {} with a weapon rating of {}.",
        winner_name, winner_rating,
    );
}

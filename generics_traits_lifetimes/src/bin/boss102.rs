use std::fmt::Debug;

fn main() {
    let sword = Sword { physical_damage: 5 };

    let staff = Staff { magical_damage: 5 };

    duel(&sword, &staff);
}

pub trait Weapon {
    fn weapon_rating(&self) -> u32 {
        5 // default damage, but how do I even use this if a struct requires its damage?
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

#[derive(Debug)]
pub struct Sword {
    physical_damage: u32,
}

impl Weapon for Sword {
    fn weapon_rating(&self) -> u32 {
        self.physical_damage * 2
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
    fn weapon_rating(&self) -> u32 {
        self.magical_damage * 3 // slightly higher as mages have slightly lower HP
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
    // compare which weapon rating is higher
    if primary.weapon_rating() > secondary.weapon_rating() {
        println!(
            "A duel has begun!\n
    Contestant 1: {primary:?}\n
    Contestant 2: {secondary:?}\n
    The winner is {} with a weapon rating of {}.",
            primary.weapon_name(),
            primary.weapon_rating(),
        );
    } else {
        println!(
            "A duel has begun!\n
    Contestant 1: {primary:?}\n
    Contestant 2: {secondary:?}\n
    The winner is {} with a weapon rating of {}.",
            secondary.weapon_name(),
            secondary.weapon_rating(),
        );
    };
}

fn main() {
    // let potion: Potion = Potion {
    //     name: String::from("Healing Potion"),
    //     effect: String::from("Healing"),
    // };

    let mut hero: Hero = Hero {
        hp: 100,
        mp: 50,
        status: None,
        inventory: vec![Item::Potion(Potion {
            name: String::from("Healing Potion"),
            effect: String::from("Healing"),
        })],
    };

    // println!("{potion:?}"); // was moved into hero.inventory, so invalid

    // Fn Test
    let ready_check = || {
        if hero.hp < 50 || hero.mp < 25 {
            println!("The hero could use some potions! {}", hero.total_check());
        } else if hero.hp == 0 {
            println!("The hero is dead! {}", hero.total_check());
        } else {
            println!("The hero is ready for battle! {}", hero.total_check());
        }
    };

    process_aura_check(ready_check); // simply does ready_check 3 times

    // FnMut Test
    let mut total_damage_dealt: u32 = 0;

    let battle = || {
        hero.hp -= 25;
        println!("Hero took 25 damage! {}", hero.hp_check());
        total_damage_dealt += 25;
    };

    // process_combat_loop(battle, total_damage_dealt); // doesn't work as it mutably borrows and tries to immutably borrow for print
    process_combat_loop(battle); // NLL states that mutable borrow ends here
    println!("Total damage dealt to Hero: {}", total_damage_dealt); // so I can immutably borrow here with no issue!

    // FnOnce Test
    let consume_item = || {
        let item = hero.inventory.pop().expect("Inventory is empty!");
        item
    };

    process_consumable(consume_item);
    // process_consumable(consume_item); // use of moved value, which emphasizes the fact that you can only pull the trigger ONCE
}

pub struct Hero {
    pub hp: u32,
    pub mp: u32,
    status: Option<Status>,
    inventory: Vec<Item>,
}

impl Hero {
    fn hp_check(&self) -> String {
        format!("HP: {}", self.hp)
    }

    fn mp_check(&self) -> String {
        format!("MP: {}", self.mp)
    }

    fn total_check(&self) -> String {
        format!("{}, {}", self.hp_check(), self.mp_check())
    }
}

#[derive(Debug)]
struct Potion {
    name: String,
    effect: String,
}

#[derive(Debug)]
struct Weapon {
    name: String,
    damage: u32,
}

#[derive(Debug)]
struct Armor {
    name: String,
    defense: u32,
}

enum Status {
    Burning,
    Poisoned,
    Frozen,
}

#[derive(Debug)]
enum Item {
    Potion(Potion),
    Weapon(Weapon),
    Armor(Armor),
}

// The function won't actually return a String: the CLOSURE passed into the function RETURNS A STRING
// If we wanted the outer function to return the String, we could just add the return type AFTER the parameter parentheses
fn process_consumable<F: FnOnce() -> Item>(func: F) {
    println!("Consumed: {:?}", func()); // func() returns the Item, which we then use in this function!
}

// fn process_combat_loop<F: FnMut()>(mut func: F, total_dmg: u32) {
//     for i in 1..4 {
//         // Repeat 3 times with a mutable reference
//         func();
//     }

//     println!("{}", total_dmg)
// }

fn process_combat_loop<F: FnMut()>(mut func: F) {
    for i in 1..4 {
        // Repeat 3 times with a mutable reference
        func();
    }
}

fn process_aura_check<F: Fn()>(func: F) {
    for i in 1..4 {
        // Repeat 3 times with an immutable reference
        func();
    }
}

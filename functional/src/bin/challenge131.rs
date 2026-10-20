use std::thread;

#[derive(Debug)]
struct Character {
    hp: u32,
    mp: u32,
    dmg: u32,
}

impl Character {
    fn compute_combat_power(&self) -> u32 {
        self.hp + self.mp + self.dmg
    }
}

fn main() {
    // 1. FnOnce Consumable
    let fireball_scroll: String = String::from("Fireball Scroll");
    println!("I have a {fireball_scroll}!");
    let use_item = move || {
        println!("Using Fireball Scroll! (called closure)");
        fireball_scroll
    };

    let consumed_item: String = use_item();
    println!("I used {consumed_item}!\n");

    // Failure Conditions:
    // use_item(); // use of moved value
    // record_kills(use_item); // issue: record_kills expects a return of unit type (), not String (fireball_scroll)

    // 2. FnMut Kill Counter
    let mut monsters_slain: u32 = 0;
    let monsters_slain_vector: Vec<String> = vec![
        String::from("Goblin"),
        String::from("Skeleton"),
        String::from("Dragon"),
        String::from("Dark Mage"),
        String::from("Zombie"),
    ];
    println!("I have killed {monsters_slain} monsters.");
    let mut add_monsters_slain = || {
        for monster in &monsters_slain_vector {
            // immutable reference to vector because we don't want to consume it
            monsters_slain += 1;
            println!("Recording kill: {monster}");
        }
    };
    // add_monsters_slain();
    record_kills(&mut add_monsters_slain);
    // println!("{monsters_slain}"); // fails as it is borrowed mutably afterward, when it requires an immutable borrow here
    // add_monsters_slain(); // NLL occurs here
    record_kills(&mut add_monsters_slain);
    println!("I have successfully recorded {monsters_slain} monster kills!"); // can borrow immutably now as NLL occurred before

    // 3. Fn Aura Reader
    let character: Character = Character {
        hp: 30,
        mp: 50,
        dmg: 10,
    };
    let message = || {
        println!(
            "The character has {} HP and {} MP. They can deal {} DMG. Their total combat power is {}.",
            character.hp,
            character.mp,
            character.dmg,
            character.compute_combat_power(),
        );
    };
    message();
    message();
    aura_read_thrice(message);

    // 4. Off-thread Event Logger
    println!("Before defining closure: {character:#?}");
    thread::spawn(move || {
        /*
        Deleting move results in: closure may outlive the current function, but it borrows `character`,
        which is owned by the current function may outlive borrowed value `character`

        In other words, creating a thread and expecting unmoved data to live as long as the closure is invalid as they are
        in different scope stacks
        */
        println!("Now in new thread: {character:#?}");
    })
    .join()
    .unwrap();
}

// Functions below are a bit hard to memorize aside their trait bound
fn aura_read_thrice<F: Fn()>(read: F) {
    // Intended only for reading thrice
    read();
    read();
    read();
}

fn record_kills<F: FnMut()>(mut record: F) {
    record();
}

fn consume_once<F: FnOnce() -> String>(consume: F) {
    let result = consume();
    println!("{result}");
}

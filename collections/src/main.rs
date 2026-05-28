fn main() {
    // Empty, mutable loot pool for items to spawn
    let mut loot_pool: Vec<String> = Vec::new();

    // Loot pool filling
    loot_pool.push(String::from("Iron Sword"));
    loot_pool.push(String::from("Health Potion"));
    loot_pool.push(String::from("Gold Coin"));

    // Client attempts to query an item at index 5
    let item: Option<&String> = loot_pool.get(5);
    match item {
        // whichever .get() returns
        Some(item) => println!("Rare item found: {}", item),
        None => println!("Slot empty."),
    }

    // For every item in the loot pool
    for i in &mut loot_pool {
        // NO NEED TO DEREFERENCE as you can auto-dereference
        // with the dot operator method call on a reference
        i.push_str(" (Cursed)"); // &str (slice) not new String
    }

    // Auto-loot most recent item therefore
    // remove last item and take ownership of it
    let player_loot: String = loot_pool.pop().unwrap(); // unwrap Some(String)
    println!("Player looted: {}", player_loot);

    // Iterate immutably and print
    for i in &loot_pool {
        println!("Remaining loot: {}", i);
    }
}

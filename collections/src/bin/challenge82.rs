fn main() {
    let mut quest: String = String::from("Objective: ");
    quest.push_str("Slay the ");
    quest.push('🐉');

    let location: String = String::from(" in the dark caves.");

    // quest was moved into full_quest
    // location was immutably borrowed for the operation
    let full_quest: String = quest + &location;

    let ui_display = format!("[ACTIVE] {full_quest}");

    println!("Byte count: {}", ui_display.len()); // outputs 52

    let mut char_count: i32 = 0;
    for _char in ui_display.chars() {
        char_count += 1;
    }

    println!("{}", ui_display);
    println!("True Character Count: {}", char_count); // outputs 49

    // so the dragon is presumably 4 bytes, from 49 (1 byte) to 52 (4 bytes), correct?
}

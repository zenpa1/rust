fn main() {
    // Loop repeats forever until explicitly stopped
    // loop {
    //     println! {"again"};
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // The value to return out of loop
        }
    };

    println!("The result is {result}");

    // Loops within loops will have break and continue apply
    // to the innermost loop

    // Optional loop label can specify which loop uses break/continue

    let mut count = 0;
    'counting_up: loop {
        // Count from 0 to 2
        println!("count = {count}");

        // Part of the loop, so it re-initializes to 10 after loop reset
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            // Set 9 as remaining limit
            if remaining == 9 {
                break; // No label: break inner loop
            }

            // Set 2 as count limit
            if count == 2 {
                break 'counting_up; // Labeled: break labeled loop
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // while loop
    // While condition is true, run loop
    // Otherwise, call break
    // while loops can be implemented with a combination of
    // loop, if, else, and break
    // But this is so common so it has its own construct

    let mut health = 3;

    while health != 0 {
        println!("HP: {health}!");

        health -= 1;
    }

    println!("YOU DIED");

    // Can also be used for collections
    // ... but it is error prone
    // What if index val is incorrect?
    // What if test condition is incorrect?
    // Use for loops instead
    let inventory = ["book", "potion", "bread"];
    let mut index = 0;

    while index < 3 {
        println!("the current item is: {}", inventory[index]);

        index += 1;
    }

    // for loop
    // Most common loop construct
    for item in inventory {
        println!("The item shown is: {item}");
    }

    for num in (1..4).rev() {
        println!("{num}");
    }
}

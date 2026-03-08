fn main() {
    // Array for gifts
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    // Array for ordinal numbers
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // Index for going through days
    let mut day_index: usize = 0;

    // Maximum lengths of array
    let day_array_length = days.len();

    // Iterate through days
    'day_loop: loop {
        let mut gift_index = day_index;

        print!(
            "On the {} day of Christmas, my true love gave to me: ",
            days[day_index]
        );

        // Iterate through gifts
        'gift_loop: loop {
            if gift_index == 0 && day_index == 0 {
                // For day 0, where it is one gift
                print!("{}.", gifts[0]); // print! is println! but without the newline
                break 'gift_loop;
            } else if gift_index == 0 {
                // For other days with multiple gifts
                print!("and {}.", gifts[0]);
                break 'gift_loop;
            }

            // Repeatedly print gifts
            print!("{}, ", gifts[gift_index]);

            // Reverse order
            gift_index -= 1;
        }

        // Go through days
        day_index += 1;

        // Prevent out of bounds
        if day_index == day_array_length {
            break 'day_loop;
        }

        println!("\n")
    }

    // Better for preventing out of bounds errors
    // for day_index in 0..days.len() {
    //     // day_index is automatically 0, 1, 2...
    //     for gift_index in (0..=day_index).rev() {
    //         // gift_index goes from day_index down to 0
    //     }
    // }
}

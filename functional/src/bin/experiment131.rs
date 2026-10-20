use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| {
            println!("No preference found. Opting for most stocked...");
            self.most_stocked()
        })
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        // Loop through inventory.shirts
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situpts!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity))
        };
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;

    // generate_workout(simulated_user_specified_value, simulated_random_number);

    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");

    // // Immutable borrow
    // let only_borrows = || println!("From closure: {list:?}");

    // // Mutable borrow
    // let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {list:?}");
    // only_borrows();

    // borrows_mutably();
    // borrows_mutably();
    // borrows_mutably();

    // // Mutable borrow experiment
    // let mut borrows_mutably = |mut list: Vec<u32>| list.push(67);
    // println!("Before calling closure: {list:?}");

    // borrows_mutably(list);

    // println!("After calling closure: {list:?}");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Prints the vector in a new thread rather than in the main thread
    // In this case, we give the thread a closure to run as an argument
    // If we remove move keyword:
    // closure may outlive the current function, but it borrows `list`, which is owned by the current function
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // Already consumed/moved; won't compile
    // println!("{list:?}");

    let mut rect_list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // // sort_by_key() uses FnMut instea of FnOnce because it calls the closure multiple times:
    // // once for each item in the slice
    // // Does not capture, mutate, or move anything out from its environment, so meets trait bound requirements
    // rect_list.sort_by_key(|r| r.width); // Sorts the slice in ASCENDING order
    // println!("{rect_list:#?}");

    // // In contrast, an example using the `FnOnce` trait as it moves a value out of its environment
    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    // rect_list.sort_by_key(|r| {
    //     // captured by this FnMut closure
    //     sort_operations.push(value); // value is moved here
    //     /*
    //     In other words, when we call it the first time, the closure executes and push(value)
    //     takes full ownership of the String and moves it out of the closure into sort_operations

    //     However, calling it the second time (because once every item), the value is already gone

    //     Because the code inside attempts to move value out, Rust marks this closure as
    //     FnOnce only, so handing this FnOnce closure to sort_by_key which expects an FnMut stops you
    //     immediately
    //     */
    //     r.width
    // });

    // println!("rect_list:#?");

    // This version works because integers implement the Copy trait, meaning we just add
    // instead of moving a String via ownership (still FnMut because the closure gets called multiple times)
    let mut num_sort_operations = 0;
    rect_list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{rect_list:#?}, sorted in {num_sort_operations} operations");

    let mut a = String::from("Hello");
    let b = &mut a;
    println!("{b}");
    b.push_str("world ");
    println!("{a}");
}

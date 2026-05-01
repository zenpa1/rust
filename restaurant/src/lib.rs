// mod front_of_house {
//     // front_of_house module
//     // that contains other modules
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn _seat_at_table() {}
//     }

//     mod serving {
//         fn _take_order() {}

//         fn _serve_order() {}

//         fn _take_payment() {}
//     }
// }

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }

// // pub marks functions as part of library crate's public API
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // user can pick toast
        seasonal_fruit: String, // but only chef can pick fruit based on season
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast), // wait, a reference to make a String?
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // order breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // this next line won't compile as we are not allowed
    // to see or modify the seasonal fruit
    // meal.seasonal_fruit = String::from("blueberries");
}

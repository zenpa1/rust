// match vs. if let
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// // might be difficult to keep up with
// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     if let Coin::Quarter(state) = coin {
//         if state.existed_in(1900) {
//             Some(format!("{state:?} is pretty old, for America!"))
//         } else {
//             Some(format!("{state:?} is relatively new."))
//         }
//     } else {
//         None
//     }
// }

// // annoying alternative so not worth it
// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let state = if let Coin::Quarter(state) = coin {
//         state // return value as expression
//     } else {
//         return None; // I'm not sure why this requires a return keyword
//     };

//     /*
//     I assume we are simply trying to separate the logic so it
//     becomes more readable, there's no way this is a way of
//     using let state as a way to extend the lifetime for the
//     code in the bottom to use... right?
//     */
//     if state.existed_in(1900) {
//         Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }

// let else alternative
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    /*
    The book explained it in such a confusing manner. But, from
    what I understand, looking at the syntax alone:

    IF the pattern on the left matches 'coin' (expression on the right),
    then bind

    ELSE return None; // although I'm still not sure why it requires a keyword
    */

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let coin = Coin::Penny;
    let config_max = Some(3u8); // Assigning the value 3 as an unsigned 8-bit 
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (), // boilerplate if you just need to process 1 variant
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
        /*
        Pattern is its first arm [Some(max)]
        max binds to the value inside the Some()
        We can use max in the body of the if let block the same way
        we would use it in match
        */
    }

    let mut count = 0;
    // // another example
    // match coin {
    //     Coin::Quarter(state) => println!(),
    //     _ => count += 1,
    // }

    // alternative
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}

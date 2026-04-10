// struct { fields }
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn main() [
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");
// ]

// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );

//     let user2 = User {
//         email: String::from("anotheruser@example.com"),
//         ..user1 // must come last to specify that
//                 // remaining fields should get their values
//                 // from fields in user1
//     };
// }

// // // too much work on repeating the username and email
// // fn build_user(input_email: String, input_username: String) -> User {
// //     User {
// //         active: true,
// //         username: input_username,
// //         email: input_email,
// //         sign_in_count: 1,
// //     }
// // }

// // field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // only works if same name as param
        email,
        sign_in_count: 1,
    }
}

// // tuple structs
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// // p and p.x lose perms but not p.y
// fn main() {
//     struct Point {
//         x: i32,
//         y: i32,
//     }

//     let mut p = Point { x: 0, y: 0 };

//     let x = &mut p.x;

//     *x += 1;

//     println!("x: {}, y: {}", p.x, p.y)
// }

// breaking stuff
fn main() {
    // try to use user1 after creating user2
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user2 = User {
        email: String::from("anotheruser@example.com"),
        ..user1
    };

    // this will break as user1's username was moved
    // into user2's username (no Copy trait)
    println!("User1 active: {}", user1.active); // works fine
    // why? partial move, we don't use the moved value
    // println!("User1 username: {}", user1.username); // boom

    struct Point(i32, i32, i32);

    let origin = Point(1, 2, 3);

    let Point(x, y, z) = origin;
    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");
}

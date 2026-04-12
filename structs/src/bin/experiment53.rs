// // remove area fn and instead make an area method
// // defined on the Rectangle struct
// #[derive(Debug)]
#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// // we can name a method the same as a field
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn width(&self) -> bool {
//         self.width > 0
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }

//     fn set_width(&mut self, width: u32) {
//         self.width = width;
//     }
// }

// // multiple impl blocks can exist
// // each method is in its own impl block
// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };

//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     // method call
//     if rect1.width() {
//         // method access
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }

//     println!("The area of rect1 is: {}", rect1.area());

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

//     // associated function
//     let my_square = Rectangle::square(3);
//     println!("The area of my_square is {}", my_square.area());
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn set_width(&mut self, width: u32) {
//         self.width = width;
//     }
// }

// fn main() {
//     // let mut r = Rectangle {
//     //     width: 1,
//     //     height: 2,
//     // };

//     // let area1 = r.area();
//     // let area2 = Rectangle::area(&r);
//     // assert_eq!(area1, area2);

//     let r = &mut Box::new(Rectangle {
//         width: 1,
//         height: 2,
//     });

//     let area1 = r.area();
//     let area2 = Rectangle::area(&**r);
//     assert_eq!(area1, area2);

//     r.set_width(2);
//     Rectangle::set_width(&mut r, 2);
//     assert_eq!(area1, area2);
// }

// struct Point(i32, i32);
// fn main() {
//     let p = Point(1, 2);

//     impl Point {
//         fn x(&self) -> i32 {
//             self.0
//         }
//     }

//     println!("{}", p.x());
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn set_width(&mut self, width: u32) {
//         self.width = width;
//     }

//     fn max(self, other: Rectangle) -> Rectangle {
//         Rectangle {
//             width: self.width.max(other.width),
//             height: self.height.max(other.height),
//         }
//     }
// }

// fn main() {
//     let rect = Rectangle {
//         width: 0,
//         height: 0,
//     };

//     println!("{}", rect.area()); // uses immutable ref (read)
//     let other_rect = Rectangle {
//         width: 1,
//         height: 1,
//     };

//     let max_rect = rect.max(other_rect);

//     println!("width: {}, height: {}", max_rect.width, max_rect.height);

//     // write perm
//     let mut mutable_rect = Rectangle {
//         width: 0,
//         height: 0,
//     };

//     mutable_rect.set_width(1);
//     let rect_ref = &mut mutable_rect;
//     rect_ref.set_width(2);

//     println!("{}", mutable_rect.width);
// }

// // good and bad moves
// // can compiler set_to_max by adding this:
// impl Rectangle {
//     fn max(self, other: Self) -> Self {
//         let w = self.width.max(other.width);
//         let h = self.height.max(other.height);

//         Rectangle {
//             width: w,
//             height: h,
//         }
//     }

//     fn set_to_max(&mut self, other: Rectangle) {
//         let max = self.max(other);
//         *self = max;
//     }
// }

// fn main() {
//     let mut rect = Rectangle {
//         width: 0,
//         height: 1,
//     };

//     let other_rect = Rectangle {
//         width: 1,
//         height: 0,
//     };

//     rect.set_to_max(other_rect);
// }

// more relatable good and bad moves example
// 1. We tell Rust it's okay to duplicate this struct if we explicitly ask for it
#[derive(Clone)]
struct Sword {
    name: String,
    damage: u32,
}

impl Sword {
    /*
    takes 'self' (non-reference); completely consumes original
    equivalent of the book's `fn max(self, other: Self)`
    */

    // you can refactor this to just use a mutable borrow instead of self, avoiding the need for clone
    fn absorb_magic(self, sacrifice: Sword) -> Sword {
        let new_damage = self.damage + sacrifice.damage;

        Sword {
            name: String::from("Awakened Sword"),
            damage: new_damage,
        } // The old `self` and `sacrifice` are destroyed here, and a new sword is returned
    }

    // only BORROW sword here (&mut self)
    fn upgrade_on_anvil(&mut self, sacrifice: Sword) {
        // error
        // *self = self.absorb_magic(sacrifice);

        /*
        the reason this won't work is because self and
        sacrifice are consumed, so what exactly is *self
        for a millisecond in the code?

        if something happens in that one millisecond,
        it would crash the program immediately if
        main would need that sword again

        ALSO, it is &mut self, a mutable ref, not
        just self, which does not make sense for
        a function signature when the function itself
        needs ownership, right?
        */

        // fix by clone
        *self = self.clone().absorb_magic(sacrifice);
    }
}

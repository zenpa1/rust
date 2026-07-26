// // These two function bodies have the same code, so how do we get rid of duplication?
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T,
//     // y: U, // can be different than T
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x // getter
//     }

//     fn y(&self) -> &T {
//         &self.y
//     }
// }

// impl Point<i32> {
//     // duplicate definitions error
//     fn x(&self) -> &T {
//         println!("hello");
//         &self.x
//     }
// }

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {result}");

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {result}");

    // let integer: Point<i32> = Point { x: 5, y: 10 };
    // let float: Point<f64> = Point { x: 1.0, y: 4.0 };

    // println!("integer.x = {}", integer.x());
    // println!("float.y = {}", float.y());

    // Point<T> is generic over some type T, and both x and y are BOTH THAT SAME TYPE
    // but if another generic type was involved (U), it's OK
    // let wont_work = Point { x: 5, y: 4.0 };

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // println!("p2.x = {}", p2.x); // already moved
}

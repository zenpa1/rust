use std::ops::Deref;

fn main() {
    // let x = 5;
    // // let y = &x;
    // let y = Box::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // // assert_eq!(5, y);

    // With user-defined struct:
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String>
    // *provided it implements the Deref trait
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

struct MyBox<T>(T); // tuple struct with one element of type T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x) // return a MyBox instance that holds the value passed in
    }
}

impl<T> Deref for MyBox<T> {
    // Without this trait, the compiler can only dereference & references

    type Target = T; // defines an associated type for the Deref trait to use
    // Associated type - slightly different way of declaring a generic parameter

    fn deref(&self) -> &Self::Target {
        &self.0 // returns a reference to the value we want to access with the * operator
        // .0 accesses the first value in a tuple struct
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

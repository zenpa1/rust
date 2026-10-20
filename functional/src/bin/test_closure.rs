fn main() {
    test_closure();
}

fn test_closure() {
    // Fn example
    let test_print = || println!("works");
    test_print();

    // FnMut example
    let mut a = 5; // declared mut because its value changes
    let mut test_add = || {
        // declared mut because calling the closure requires a mutable borrow
        a += 5;
        println!("{a}");
    };
    test_add();

    // Fn example
    let str = String::from("Hello world!");
    let test_move = move || str; // explicit move keyword but works without
    let result = test_move();
    // println!("{str}"); // moved
    println!("{result}");
}

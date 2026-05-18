fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    let mut my_vec = vec![1, 2, 3, 4, 5];

    // indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // get method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &my_vec[100]; // panics: index out of bounds
    let does_not_exist = my_vec.get(100); // ok: returns Some(&element) or None

    let first = &my_vec[0]; // immutable borrow

    // my_vec.push(6); // mutable borrow

    println!("The first element is: {first}"); // immutable borrow used here

    let iter_vec = vec![100, 32, 57];
    for i in &iter_vec {
        println!("{i}");
    }

    let mut mut_iter_vec = vec![100, 32, 57];
    for i in &mut mut_iter_vec {
        *i += 50;
        println!("{}", *i);
    }
}

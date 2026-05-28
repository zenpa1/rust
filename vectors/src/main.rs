fn main() {
    use std::slice::Iter;

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

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]

    let mut vec_iter: Vec<i32> = vec![1, 2];
    let mut iter: Iter<'_, i32> = vec_iter.iter();
    // does not copy the vector; creates an iterator struct
    // holds 2 raw pointers to the vector's heap allocation
    // one to current element; one to end of slice
    // also, .iter() registers an immutable borrow

    /*
    iter is a pointer that moves through each element of the vector
    next advances the iterator and RETURNS an OPTIONAL REFERENCE,
    either Some (to .unwrap) or None at the end of the vector
    */

    // n1 now points to vec_iter[0]
    // we unwrap Some(1)
    let n1: &i32 = iter.next().unwrap();

    // n2 now points to vec_iter[1]
    // we unwrap Some(2)
    let n2: &i32 = iter.next().unwrap();
    
    // end is None
    // as .next() returned None at the end of the vector
    let end: Option<&i32> = iter.next();

    /*
    Considering n1 and n2 are holding active references to data
    within vec_iter, the borrow checker locks the vector, therefore,
    we cannot push into it (might exceed capacity and reallocate)
    */

}

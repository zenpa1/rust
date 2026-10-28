fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    // iter demo
    let v2 = vec![1, 2, 3];

    let mut v2_iter = v2.iter();

    println!("{:?}", v2_iter.next().unwrap());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());

    // consuming adapter
    let v3 = vec![1, 2, 3];

    let v3_iter = v3.iter();
    
    let total: i32 = v3_iter.sum();

    println!("{}", total);
    println!("{:?}", v3);
    // println!("{:?}", v3_iter) // borrow of moved value as sum took ownership

    // iterator adapter
    let v4: Vec<i32> = vec![1, 2, 3];

    // collect transforms an iteration into a collection by consuming it
    let v5: Vec<_> = v4.iter().map(|x| x + 1).collect();

    println!("v4: {:?} -> v5: {:?}", v4, v5);

    // borrowing and not consuming the iterator
    let mut numbers = vec![10, 20, 30 ,40];
    let mut numbers_iter = numbers.iter();

    // Use iterator to get one item but NOT consume it
    if let Some(first) = numbers_iter.next() {
        println!("First item: {first}");
    }

    // numbers_iter is STILL VALID and we can use a consuming adapter on the rest
    let total: i32 = numbers_iter.sum();
    println!("Remaining sum: {total}");

    // partial consumption (using consuming adapter but not taking ownership)
    let mut items = vec!["sword", "shield", "potion", "scroll"];
    let mut items_iter = items.iter();

    // Consume first 2 items into a vector by borrowing items_iter
    // by_ref() lets you consume a slice of the elements WITHOUT consuming the
    // iterator object itself
    let first_two: Vec<_> = items_iter.by_ref().take(2).collect();
    println!("First two: {first_two:?}");

    // iter was NOT consumed so it continues from item # 3
    let next_item = items_iter.next();
    println!("Third item: {next_item:?}");

    // peeking (reading without consuming elements)
    // looking at an element without advancing the iterator pointer at all
    let numbers2 = vec![1, 2, 3];
    let mut numbers2_iter = numbers2.iter().peekable();

    // inspect next element without consuming or advancing it
    if let Some(&&first) = numbers2_iter.peek() {
        println!("Peeked at: {first}");
    }

    // item is still there
    println!("{:?}", numbers2_iter.next());
;}
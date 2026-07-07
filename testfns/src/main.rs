fn main() {
    println!("Hello, world!");

    let mut vec = vec![1, 2, 3, 0];
    remove_zeros(&mut vec);

    for integer in vec {
        println!("{}", integer);
    }
}

fn remove_zeros(v: &mut Vec<i32>) {
    for i in (0..v.len()).rev() {
        if v[i] == 0 {
            v.remove(i);
            v.shrink_to_fit();
        }
    }
}

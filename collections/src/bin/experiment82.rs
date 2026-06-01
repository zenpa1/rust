fn main() {
    let mut s = String::new(); // create an instance

    let data = "initial contents";

    let s = data.to_string();

    // method also works on a literal directly
    let s = "literal direct".to_string();

    println!("{s}");

    let mut a: String = String::from("exo");
    a.push_str("tic");

    println!("{a}");

    let mut s1: String = String::from("extermi");
    let s2: &str = "nation";
    s1.push_str(s2); // does not move
    println!("s2 is: {s2}");

    let mut b: String = String::from("lma");
    b.push('o');
    println!("{b}");

    let s3: String = String::from("hello, ");
    let s4: String = String::from("world!");
    let s5: String = s3 + &s4; // s1 was moved but not s2
    println!("{s5} + {s4}");

    let s7: String = String::from("tic");
    let s8: String = String::from("tac");
    let s9: String = String::from("toe");

    // let s10: String = s7 + "-" + &s8 + "-" + &s9;
    // println!("{s10}");

    let s11: String = format!("{s7}-{s8}-{s9}");
    println!("{s11}");
}

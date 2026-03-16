// fn read(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }

// fn main() {
//     // unsafe ver.
//     // read(x); // x is not defined

//     /*
//     The goal of Rust is to compile programs into efficient binaries
//     that require as few runtime checks as possible.

//     Rust does NOT check at runtime, but rather, at compile-time.

//     > Avoids bugs in production
//     > Fewer runtime checks (improved performance)

//     A foundational goal of Rust is to ensure programs never have
//     undefined behavior.
//     */
//     let x = true;
//     read(x);
// }

// fn main() {
//     let n = 5;
//     let y = plus_one(n);
//     println!("The value of y is {y}");

//     // There are two boxes containing 15 in the heap
//     let a = Box::new(15);
//     let b = a; // Points to box that was pointed by a
//     let c = Box::new(15);

//     let d = 5;
//     let e = 10;
//     // Assert that x is less than y
//     assert!(x < y);
//     // Uncommenting the line below will cause a panic with a default message
//     // assert!(x > y);
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn main() {
//     let first = String::from("Ferris");
//     let first_clone = first.clone(); // Clone to prevent moving data
//     let full = add_suffix(first_clone);
//     println!("{full}, originally {first}");
// }

// fn add_suffix(mut name: String) -> String {
//     name.push_str(" Jr.");
//     name
// }

// fn main() {
//     let mut first = String::from("Ferris"); // Note the 'mut'

//     let full = add_suffix(first);
//     // 'first' is now moved and invalid. If you print it here, it fails.

//     first = String::from("Rustacean");
//     // We gave it a new allocation! 'first' is valid again.

//     println!("{full}, and the new first is {first}");
// }

// fn main() {
//     let s = String::from("hello");
//     let s2;
//     let b = false;
//     if b {
//         s2 = s;
//     }

//     println!("{}", s);
// }

// fn main() {
//     let m1: String = String::from("Hello");
//     let m2: String = String::from("world");
//     // let (m1_again, m2_again) = greet(m1, m2);
//     // ^ You can instead opt to shadow it

//     let (m1, m2) = greet(m1, m2);

//     // let s: String = format!("{} {}", m1_again, m2_again);
//     // ^ You can instead opt to shadow it

//     let s: String = format!("{} {}", m1, m2);
//     println!("{}", s);
// }
// fn greet(g1: String, g2: String) -> (String, String) {
//     println!("{} {}!", g1, g2);
//     (g1, g2)
// }

// fn main() {
//     // References are NON-OWNING POINTERS as they do not
//     // own the data they point to
//     let m1: String = String::from("Hello");
//     let m2: String = String::from("world");

//     /*
//     Ampersand operator (&) creates a reference to (or "borrow") m1

//     &String means "a reference to a String"
//     */
//     greet(&m1, &m2);
//     let s = format!("{} {}", m1, m2);
//     println!("{}", s);
// }

// fn greet(g1: &String, g2: &String) {
//     /*
//     g1 is a reference that points to m1 on the stack
//     and m1 is a String containing a box that points to
//     "Hello" on the heap

//     g1 does NOT own either m1 or "Hello", which means
//     Rust does not deallocate "Hello" on behalf of g1

//     After greet ends, no heap data is deallocated:
//     only the stack frame for greet disappears

//     ^ Consistent with Box Deallocation Principle
//     */
//     println!("{} {}!", g1, g2);
// }

// fn main() {
//     // Dereferencing (*)
//     let mut x: Box<i32> = Box::new(1);
//     let a: i32 = *x; // *x reads the heap value, so a = 1
//     *x += 1; // *x on the left-side modifies the heap value,
//     // so x points to the value 2

//     let r1: &Box<i32> = &x; // r1 points x on the stack
//     let b: i32 = **r1; // two dereferences get us to the heap value

//     let r2: &i32 = &*x; // r2 destructures x, pointing to the heap value directly
//     let c: i32 = *r2; // only one dereference is needed to read it
// }

// fn main() {
//     let x: Box<i32> = Box::new(-1);
//     let x_abs1: i32 = i32::abs(*x); // explicit dereference
//     let x_abs2: i32 = x.abs(); // implicit dereference
//     assert_eq!(x_abs1, x_abs2);

//     let r: &Box<i32> = &x;
//     let r_abs1: i32 = i32::abs(**r); // explicit dereference (twice)
//     let r_abs2: i32 = r.abs(); // implicit dereference (twice)
//     assert_eq!(r_abs1, r_abs2);

//     let s: String = String::from("Hello");
//     let s_len1: usize = str::len(&s); // explicit reference
//     let s_len2: usize = s.len(); // implicit reference
//     assert_eq!(s_len1, s_len2);
// }

// fn main() {
//     /*
//     Arrays have fixed length
//     Vectors have variable length as their elements are stored in the heap
//     */
//     let mut v: Vec<i32> = vec![1, 2, 3]; // vec! is a macro
//     v.push(4);

//     // Data CAN be aliased. Data CAN be mutated.
//     // ...but not both
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];

//     let num: &i32 = &v[2];

//     println!("The third element is {}", *num);
//     println!("Again, it is {}", *num);
//     // ^ Instead of dropping num after the first println!,
//     // we can do it after its full usage
//     // It's a problem if we use num again AFTER mutating v, though

//     v.push(4);
// }

// fn main() {
//     // init v with R+W+O
//     let mut v: Vec<i32> = vec![1, 2, 3];

//     // init num with R+O and *num with R+W
//     let num: &mut i32 = &mut v[2];

//     *num += 1;

//     // after call, num and *num loses its perms
//     println!("Third element is {}", *num);

//     // given perms before ultimately loing them again
//     println!("Vector is now {:?}", v);
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];
//     let num: &mut i32 = &mut v[2];
//     *num += 1;
//     let num2: &i32 = &*num; // Mutable ref can be "downgraded" to R-only
//     println!("{} {}", *num, *num2);
// }

// fn main() {
//     let mut x: i32 = 1;

//     // lifetime of y starts here
//     let y: &i32 = &x;

//     // and ends here
//     let z: i32 = *y;

//     x += z;

//     println!("{}", x);
// }

fn ascii_capitalize(v: &mut Vec<char>) {
    let c: &char = &v[0];

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}

fn main() {
    let mut vector: Vec<char> = vec!['A', 'b', 'c'];

    ascii_capitalize(&mut vector);

    println!("{:?}", vector);
}

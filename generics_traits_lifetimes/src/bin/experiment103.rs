fn main() {
    let r: &i32; // can be initialized without a value though!

    // {
    //     let x = 5;
    //     r = &x; // x does not live long enough
    // } // x is dropped here while it is still borrowed
    // in essence, the value that r refers to has gone out of scope
    // before we try to use it, leading to a compilation error!

    // this block can be commented out and it would still compile
    // because r was not used, so even if it doesn't have a value, it compiles
    let x = 5;
    r = &x;
    println!("r: {}", r);
}
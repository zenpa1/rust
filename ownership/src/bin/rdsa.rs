// attempt: Recursive Descent with Synthesized Attributes
fn main() {
    let v: Vec<char> = vec!['1', '0', '1'];
    let first: u32;
    let second: u32;
    recursive_descent(v, &first, &second);
}

fn recursive_descent(vector: Vec<char>,first: &u32, second: &u32) {
    // take current token from vector
    let current_token: Option<char> = vector.pop();

    if (current_token = '0') {
        let a: u32;
        let b: u32;
        println!("Consumed: {:?} as currtoken = 2", current_token);
        recursive_descent(vector, &a, &b);
        *first = a + 1;
        *second = b;
    }

    else if (current_token = '1') {
        let a: u32;
        let b: u32;
        println!("Consumed: {:?} as currtoken = 1", current_token);
        *first = a;
        *second = b + 1;
    }

    else {
        println!("Finished.");
    }
}
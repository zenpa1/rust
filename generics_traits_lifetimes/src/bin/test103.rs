// // Experiment 1
// fn main() {
//     let outer = String::from("hello");
//     let inner;
//     {
//         // let inner = "world!"; // string literal
//         // // hardcoded directly into the binary of our program
//         inner = "world";
//     }

//     let result = longest(&outer, &inner); // inner not found in scope
//     println!("{result}");
// }

// fn longest<'a>(outer: &'a str, inner: &'a str) -> &'a str {
//     if outer.len() > inner.len() {
//         outer
//     } else {
//         inner
//     }
// }

// // Experiment 2
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {result}");
// }

// // Experiment 3
// fn main() {
//     let string1 = String::from("hello!");
//     let string2 = "world";
//     let result;

//     result = longest(&string1, &string2);
//     drop(string2); // if string literal, does nothing!
//     // but if string, it is dropped as it is owned

//     println!("{result}");
// }

pub struct Relic<'a> { // Only 'a belongs to the struct, so only 'a goes here.
    pub inscription: &'a str,
}

impl<'a> Relic<'a> { // impl<'a, 'b> is an anti-pattern as we tie 'b to the ENTIRE impl block even if we don't use it on other fns
// Only declare lifetimes on the impl line if they belong to the struct
    // impl<'a> declares the generic lifetime variable
    // Relic<'a> applies the generic lifetime variable to Relic
    fn read_inscription(&self) -> &str {
        // Elision Rule 3: If self is a parameter, it most likely wants to return that.
        self.inscription
    }

    // Do not annotate &self with 'a here because we are telling Relic to
    // last as long as the string slice field it contains
    // And if we gave a 'static text, we make Relic immortal too...
    // Instead, it gets desugared to:
    // fn translate<'tiny, 'b>(&'tiny self, translation: &'b str) -> &'b str
    fn translate<'b>(&self, translation: &'b str) -> &'b str { // Declared 'b here as this is the only fn that utilizes it
        // Elision Rule 3: If self is a parameter, it most likely wants to return that.
        // However, we only want to return the translation, so annotate accordingly.
        translation
    } // 'b is born when translate is called and dies when translate finishes
}

fn main() {
    let relic = Relic { inscription: "Golden Statue of Liberty" };
    println!("{}", relic.read_inscription());
    println!("{}", relic.translate("Statue of Evil"));
}
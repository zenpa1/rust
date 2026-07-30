use std::fmt;
use std::fmt::Display;

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // Can call other methods in the same trait
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.summarize_author(),
            self.location
        )
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\"{}\" by {} ({})\n{}",
            self.headline, self.author, self.location, self.content
        )
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

// impl Summary for SocialPost {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// For default implentations, simply have an empty impl block
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

//
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// // Trait Bound Syntax
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// // Multiple parameters, different types
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//     println!("Breaking news! {}, along with a post: {}", item1.summarize(), item2.summarize())
// }

// // Multiple parameters, same type
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// }

// // Multiple parameters, not all requiring the &impl
// pub fn notify(item1: &impl Summary, item2: String) {
//     println!("Breaking news! {}. {}.", item1.summarize(), item2)
// }

// // Multiple trait bounds (diff types)
// pub fn notify(item: &(impl Summary + Display)) {

// }

// // Multiple trait bounds (same type)
// pub fn notify<T: Summary + Display>(item: &T) {

// }

pub fn notify(item: &(impl Summary + fmt::Display)) {
    println!("hello world! {item}");
}

// fn returns_summarizable() -> impl Summary {
//     // impl Trait in return position
//     // "return a value of some type that implements said trait"
//     SocialPost {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people",),
//         reply: false,
//         repost: false,
//     }
// }

// // will not work as impl Trait can only be used to return a single type
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             repost: false,
//         }
//     }
// }

#[derive(Debug)]
pub struct Pair<T> {
    pub x: T, // make pub for derive to print it
    pub y: T, // if not pub, cannot access to print
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// fun stuff
pub trait Character {
    // required method
    fn current_hp(&self) -> i32;

    // default method (that applies the required method)
    fn is_dead(&self) -> bool {
        self.current_hp() <= 0
    }

    fn is_bloodied(&self) -> bool {
        self.current_hp() <= 50
    }

    fn health_status(&self) -> String {
        if self.is_dead() {
            String::from("Deceased")
        } else {
            String::from("Alive")
        }
    }
}

pub struct Warrior {
    pub hp: u32,
}

impl Character for Warrior {
    fn current_hp(&self) -> i32 {
        100
    }
}

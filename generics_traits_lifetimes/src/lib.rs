// pub trait Summary {
//     fn summarize(&self) -> String;
// }

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // Can call other methods in the same trait
        format!("(Read more from {}...)", self.summarize_author())
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
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

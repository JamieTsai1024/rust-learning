// Version 1: All methods have implementations
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct SocialPost {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub repost: bool,
// }

// impl Summary for SocialPost {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// // Example of how a binary crate could use our aggregator library crate 
// // use aggregator::{SocialPost, Summary};

// fn main() {
//     let post = SocialPost {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         repost: false,
//     };

//     println!("1 new social post: {}", post.summarize());
// }

// Version 2: Default implementations 
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {}

// fn main() {
//      let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from(
//             "The Pittsburgh Penguins once again are the best \
//              hockey team in the NHL.",
//         ),
//     };
//     println!("New article available! {}", article.summarize());
// }

// Version 3: Default implementations can call other methods in the same trait 
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };
    println!("1 new social post: {}", post.summarize());
}

// Trait bound (impl Trait above is syntax sugar for this) 
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
// Can express more complexity, such as two parameters that implement Summary 
// pub fn notify(item1: &impl Summary, item2: &impl Summary) { ... }
// To force both parameters to have the same type, we need to use a trait bound
// pub fn notify<T: Summary>(item1: &T, item2: &T) { ... }

// Specify multiple trait bounds using +
// pub fn notify(item: &(impl Summary + Display)) {
// Trait bounds on generic types: use +
// pub fn notify<T: Summary + Display>(item: &T) {

// Clearer trait bounds using where clauses 
// Replace
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// With 
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

// Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

// Won't compile - you need to return a single type (can't be both SocialPost or NewsArticle)
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
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
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             repost: false,
//         }
//     }
// }

// Using trait bounds to conditionally implement methods 
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
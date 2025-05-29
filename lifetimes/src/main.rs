// Version 1: Dangling reference - won't compile since the value that r is referring to has gone out of scope before we try to use it
// Annotations show the lifetimes of the variables  
// fn main() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {r}");   //          |
// }                         // ---------+

// Version 2: Dangling reference - fix  
// fn main() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {r}");   //   |       |
//                           // --+       |
// }                         // ----------+

// Generic lifetimes in functions 
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // Lifetime annotations
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // Doesn't compile - string2 needs to be valid until the end of the outer scope 
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    // Lifetime annotations in struct definitions 
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

}

// Version 1: Lifetimes in functions - Won't compile since the return type needs a generic lifetime parameter on it 
// Rust can't tell whether the reference being returned refers to x or y  
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// Version 2: Lifetimes in functions 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Version 3: We've specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value
fn longest_1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Version 4: Doesn't compile - return value lifetime is not related to the lifetime of the parameters at all 
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// Lifetime annotations in struct definitions 
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime annotations in method definitions 
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Second and third elision rules apply - gives &self and accouncement their own lifetimes 
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// Lifetimes together (generic type parameters, trait bounds, and lifetimes)
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
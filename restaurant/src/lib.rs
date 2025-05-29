mod front_of_house {
    pub mod hosting { // pub here makes the module public, but not its contents
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    
    // Using pub for functions  
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
        
    // Using pub for enums
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Bringing paths into scope with use 
use crate::front_of_house::hosting;

pub fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
}

// Compile error: If we move eat_at_restaurant into a child module, then it's a different scope than the use statement 
// Fix: move the use within the customer module, or reference the shortcut in the parent module with super::hosting within the child customer module
// use crate::front_of_house::hosting;
// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

// Less clear (still compiles): specifying the entire path to the function 
// use crate::front_of_house::hosting::add_to_waitlist;
// pub fn eat_at_restaurant() {
//     add_to_waitlist();
// }

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); 
    }

    fn cook_order() {}

    // Using pub for structs 
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Using pub for enums (all variants are public)
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Re-exporting names 
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_3() {
    hosting::add_to_waitlist();
}

// Using nested paths 

// Before 
use std::io;
use std::io::Write;

// After 
use std::io::{self, Write};

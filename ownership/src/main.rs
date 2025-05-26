fn main() {
    // Scope of variables 
    let s = "hello";
    {                      // t is not valid here, it’s not yet declared
        let t = "hello";   // t is valid from this point forward
        // do stuff with t
    }                      // this scope is now over, and t is no longer valid

    // String literal to string
    let s = String::from("hello");

    // Mutate strings
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`

    // Scopes 
    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // Variables and data interacting with move 
    let x = 5; // bind value 5 to x
    let y = x; // copy value in x and bind it to y

    // String version 
    let s1 = String::from("hello");
    let s2 = s1; // String data is copied (pointer, length, capacity on the stack)

    // Using s1 after s2 is created won't work
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, world!");

    // Scope and assignment 
    let mut s = String::from("hello");
    s = String::from("ahoy"); // "hello" goes out of scope 
    println!("{s}, world!");

    // Deep copy using clone 
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // Stored on the stack  
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // Ownership and functions 
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function and so is no longer valid here
    // println!("{}", s);           // causes an error 
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // because i32 implements the Copy trait, x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward

    // Return values and scope 
    let s1 = gives_ownership();        // gives_ownership moves its return value into s1
    let s2 = String::from("hello");    // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    // Return multiple values using a tuple 
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");

    // Reference 
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of '{s1}' is {len}.");

    // References and borrowing
    // let s = String::from("hello");
    // change(&s); // error

    // Mutable references 
    let mut s = String::from("hello");
    change(&mut s);

    // Cannot have multiple mutable references (error)
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // Create multiple mutable references, just not simultaneous ones
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // Can only create one mutable/immutable references in a given scope (error)
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // Multiple mutable references as long as their scope doesn't overlap
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("{r3}");

    // Dangling reference 
    let reference_to_nothing = dangle();

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.
// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what it refers to, the value is not dropped.

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped, so its memory goes away. Danger!
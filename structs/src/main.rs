// Struct 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple struct 
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct 
struct AlwaysEqual;

// Storing a reference in a struct won't work without lifetimes 
// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // Creating instances from other instances with struct update syntax 
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // Equivalent 
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Tuple struct instance
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Destructure tuple struct 
    let Point(x, y, z) = origin; // name the tuple struct while destructuring 

    // Instantiate unit-like struct 
    let subject = AlwaysEqual; 

    // Storing a reference in a struct won't work without lifetimes 
    // let user1 = User {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1,
    // };
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }

    // Use field init shorthand 
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
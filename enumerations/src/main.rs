// Option 1 
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn route(ip_kind: IpAddrKind) {}

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);

//     // Using enums to type fields in a struct 
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// Option 2: More concise to represent using an enum instead of an enum inside a struct 
// enum IpAddrKind {
//     V4,
//     V6,
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// Option 3: Variants can have different types/amounts of data  
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// Option 4: Standard library definition of IP addresses
// Need to create own definition since we haven't brought the standard library's definition into scope 
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// enum with different variants 
enum Message {
    Quit,                       // no associated data 
    Move { x: i32, y: i32 },    // named fields like a struct 
    Write(String),              // single String 
    ChangeColor(i32, i32, i32), // three i32 values 
}

// Equivalent as above but with structs (more verbose)
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Methods on enums 
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // Examples of using Option values to hold number types and char types 
    let some_number = Some(5); // Option<i32
    let some_char = Some('e'); // Option<char>
    let absent_number: Option<i32> = None;

    // Won't compile: can't add i8 to Option<i8>
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
}
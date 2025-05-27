// Version 1
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Version 2: Increase readability using tuples for structure 
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Version 3: Refactor with Structs 
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );

//     // println!("rect1 is {}", rect1); // Won't work: need to implement Display 
//     println!("rect1 is {rect1:?}"); // Won't work by itself: need to implement Debug (error) - solve by adding #[derive(Debug)] in front of struct definition

//     // Using dbg!
//     let scale = 2;
//     let rect2 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect2);
// }

// fn area(rectangle: &Rectangle) -> u32 { // borrow struct using & instead of taking ownership
//     rectangle.width * rectangle.height
// }

// Version 4: Methods on structs 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Associated functions 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Getter 
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height
    }
}

// Can have multiple impl blocks per struct
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // immutable borrow to rect2 (read)
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated function without self as first parameter (not a method)
    let sq = Rectangle::square(3);
}
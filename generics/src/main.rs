fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// This will cause errors - need to restrict to types that implement PartialOrd or implement this trait 
// fn largest_generic<T>(list: &[T]) -> &T {
fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // Goal: Find the largest number in two lists

    // Version 1: Duplicate code 
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest_val = &number_list[0];
    for number in &number_list {
        if number > largest_val {
            largest_val = number;
        }
    }
    println!("The largest number is {largest_val}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest_val = &number_list[0];
    for number in &number_list {
        if number > largest_val {
            largest_val = number;
        }
    }
    println!("The largest number is {largest_val}");

    // Version 2: Extract code into a function  
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    // Version 3: Specialized functions per type 
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    // Version 4: Using generic data types 
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_generic(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_generic(&char_list);
    println!("The largest char is {result}");
    
    // Struct using generic type parameter 
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    
    // Expects int and float (error)
    // let wont_work = Point { x: 5, y: 4.0 };

    // Struct with x, y both generics but different types 
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // Methods using generic type parameter 
    let p = Point2 { x: 5, y: 10 };
    println!("p.x = {}", p.x);

    // Methods that use different generic types for the struct vs method signature 
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Specific for type f32 
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// struct Point2<T, U> {
//     x: T,
//     y: U,
// }

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

// Don't need to use the same generic type for the Point struct as the method signature 
impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

// Generic over two types and has 2 variants 
enum Result<T, E> {
    Ok(T),
    Err(E),
}
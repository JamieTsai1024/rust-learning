fn main() {
    // Integer variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    
    // Floating point numbers
    let y: f32 = 3.0; // f32

    // Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5; 

    // Boolean variables
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Character variables
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Tuple variables
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring the tuple
    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Array variables
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // An array of five elements, all initialized to 3

}
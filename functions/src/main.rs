fn main() {
    println!("Hello, world!");
    another_function();
    another_function_params(5);
    print_labeled_measurement(5, 'h');

    // Statements and expressions
    
    // let x = (let y = 6); // Will error

    let y = {
        let x = 3;
        x + 1 // No ending semicolon, so this is an expression
    };
    println!("The value of y is: {y}");

    // Functions with return values
    let x = five();
    println!("The value of x is: {x}");
    
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_params(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Adding a semicolon here would make it a statement, not an expression -> error 
}

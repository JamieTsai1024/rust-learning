fn main() {
    let v: Vec<i32> = Vec::new();

    // vec! macro 
    let v = vec![1, 2, 3];

    // Add to vector (need mut)
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading vectors 
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // panic 
    let does_not_exist = v.get(100); // returns None when panicking

    // Doesn't compile - immutable reference and mutate vector (end)
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    // Iterating over values in a vector (immutable references)
    let v = vec![100, 32, 57];
    for i in &v { 
        println!("{i}");
    }

    // Iterating over values in a vector (mutable references)
    let mut v = vec![100, 32, 57];
    for i in &mut v { 
        *i += 50;
    }

    // Vectors with multiple types using an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Dropping vector
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here
}

fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    // Equivalent 
    let s = String::from("initial contents");

    // Strings are UTF-8 encoded 
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating strings 
    let mut s = String::from("foo");
    s.push_str("bar"); // foobar 

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str doesn't take ownership of s2 
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l'); // push takes a single character, s contains "lol"

    // Concatenation 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Concatenate multiple strings 
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // s = tic-tac-toe

    // Indexing 
    // Can't use [] - Compile error  
    // let s1 = String::from("hi");
    // let h = s1[0];

    // Compile error - first byte of З is 208 and second byte is 151, but 208 isn't a valid character on its own 
    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // Use [] with a range to create a string slice containing particular bytes 
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // each character has 2 bytes, so s = Зд
    // Note: cannot slice only part of a character's bytes like &hello[0..1]

    // Iterating over strings 
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Prints 
    // З
    // д

    // Returns each raw byte 
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // Prints 
    // 208
    // 151
    // 208
    // 180
}

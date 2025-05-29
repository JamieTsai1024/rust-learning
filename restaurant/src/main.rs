// Use entire file path to bring structs into scope  
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Exception: two items with the same name in scope
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

// Alternative: using aliases
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
}

fn function4() -> IoResult<()> {
    // --snip--
}

// Using nested paths 

// Before 
use std::cmp::Ordering;
use std::io;

// After 
use std::{cmp::Ordering, io};

// Glob operator 
use std::collections::*;

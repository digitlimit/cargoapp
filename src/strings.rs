// Primitive str - Immutable fixed-length string somewhere in memory
// String  Growable - heap-allocated data structure

pub fn run() {
    let mut hello = String::from("Hello ");

    //Get length of string
    println!("hello: {}", hello.len());

    // Push only 1 chatacter
    hello.push('H');

    // Push longer string 
    hello.push_str("ello");

    println!("String: {}", hello);

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'Hello': {}", hello.contains("Hello"));

    // Replace
    println!("Replace: {}", hello.replace("Hello", "Hi"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("s:{}", s);

    // Assertion testing
    assert_eq!(2, s.len()); // pass you will see no output or error
    assert_eq!(10, s.capacity());


}
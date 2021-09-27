/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char) - its one character  
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables 
// at compile time, however, the compiler can usually infer what type we want to use based on the
// value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    //Default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: u64 = 11111111111111111;

    println!("x = {}, y = {}, z = {}", x, y, z);

    // Mutable
    let mut mutable:i8 = 12;
    mutable = 13;
    println!("Mutable new value: {}", mutable);

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);

    // Boolean
    let is_active: bool = true;
    println!("is active: {}", is_active);

    //Boolean from expression
    let is_greater: bool = 10 > 5;
    println!("Is 10 greater than 5: {}", is_greater);

    // Char 
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (a1, face));
}

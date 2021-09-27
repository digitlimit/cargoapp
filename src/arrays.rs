// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run(){
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Re-assing value
    let mut mut_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    mut_numbers[2] = 20;
    println!("{:?}", mut_numbers);

    //Get length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("{}", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &mut_numbers[1..4];
    println!("Slice: {:?}", slice);
}
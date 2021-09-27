// Vectors are resizible arrays 

use std::mem;

pub fn run(){
    let numbers: Vec<i8> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Re-assing value
    let mut mut_numbers: Vec<i8> = vec![1, 2, 3, 4, 5];
    mut_numbers[2] = 20;
    println!("{:?}", mut_numbers);

    //Get length
    println!("Vector length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i8] = &mut_numbers[1..4];
    println!("Slice: {:?}", slice);

    // Push and Pop
    mut_numbers.push(6);
    println!("After push {:?}", mut_numbers);

    mut_numbers.pop();
    println!("After pop {:?}", mut_numbers);

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in mut_numbers.iter_mut(){
        *x *= 2; //similar to javascript map, mutiply each by 2
    }
    println!("Mutated numbers: {:?}", mut_numbers);

}
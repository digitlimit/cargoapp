// variables are mutable by default, means you can't re-assign them

pub fn run() {
    let name = "Emeka";
    println!("My name is {}", name);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("Emeka", 23);
    println!("{} is {}", my_name, my_age);
}
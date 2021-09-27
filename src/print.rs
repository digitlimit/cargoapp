pub fn run() {
    println!("I love {} but {} is better", 
        "pizza", "donut");

    println!("I love {} but {} is better", 
        "pizza", "donut");

    // placeholder traits
    println!(
        "Binary for 10 is {:b}, Hex: {:x}, Octal: {:o}", 
        10, 10, 10
    );

    // tuple formatting
    println!("{:?}", ("Hello", "World", true, 1, 2.0));

    let first_name = "Chiamaka";

    // named arguments
    println!(
        "Emeka loves {first_name} {last_name}", 
        first_name = first_name, 
        last_name = "Ifeandu"
    );

    fax();
}

fn fax() {
    println!("From fax");
}
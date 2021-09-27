pub fn run() {
    let age = get_age(1983, 2021);
    greeting("Hello", "Emeka");

    println!("Your age is {}", age);

    // Closure
    let n3: i32 = 4;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("{}", add_nums(2, 3));
}

fn greeting(message: &str, name: &str){
    println!("{} {}", message, name);
}

fn get_age(dob: u16, current_year: u16) -> u16 {
    current_year - dob as u16
}


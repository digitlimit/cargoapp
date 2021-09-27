pub fn run() {
    let age = get_age(1983, 2021);
    greeting("Hello", "Emeka");
    println!("Your age is {}", age);
}

fn greeting(message: &str, name: &str){
    println!("{} {}", message, name);
}

fn get_age(dob: u16, current_year: u16) -> u16 {
    current_year - dob as u16
}
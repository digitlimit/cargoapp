// Condtitionals - Used to check the condition of something and act accordingly
pub fn run() {

    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age = true;

    //If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("You can drink");
    } else if age < 21 && check_id {
        println!("You can't drink");
    } else {
        println!("Need your ID");
    }

    //horthand if 
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);

    
}
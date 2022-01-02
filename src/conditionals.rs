pub fn run() {
    let age: u8 = 30;
    let check_id: bool = true;
    let knows_person_of_age: bool = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    //ShortHand
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of Age: {}", is_of_age);
    println!("Age in bytes: {}", std::mem::size_of_val(&age));
}

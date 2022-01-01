pub fn run() {
    let name = "shedrack";
    let mut age = 23;
    println!("my name is: {} and i am {} years old: ", name, age);
    age = 24;
    println!("{}", age);

    const USERID: i32 = 001;
    println!("My user ID is: {}", USERID);

    //Assign Multiple Variables
    let (first_name, last_name, age, country, wife) = ("Shedrack", "Abel", 23, "Nigeria", "Benita");
    println!(
        "My first name is: {} my last name is: {} i am {} years old, from {} my wife is: {}",
        first_name, last_name, age, country, wife,
    );
}

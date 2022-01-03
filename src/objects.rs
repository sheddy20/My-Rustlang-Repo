struct Person {
    first_name: String,
    last_name: String,
    middle_name: String,
}

impl Person {
    fn get_person_details(first: &str, last: &str, middle: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            middle_name: middle.to_string(),
        }
    }

    fn get_full(&self) -> String {
        format!(
            "Yo my full name is: {} {} {} Your royal highness",
            self.first_name, self.last_name, self.middle_name
        )
    }
}

pub fn run() {
    let mut p = Person::get_person_details("Shedrack", "Abel", "Nicholas");
    println!(
        "My full name is: {} {} {}",
        p.first_name, p.last_name, p.middle_name
    );
    println!("Employee -> 1: {}", p.get_full());
    p = Person::get_person_details("Timnit", "Gebru", "Jake");
    println!(
        "I am {} {} {}: AI researcher at google",
        p.first_name, p.last_name, p.middle_name
    );
    println!("Employee -> 2: {}", p.get_full());
}

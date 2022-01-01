pub fn run() {
    let person: (&str, &str, i8, bool) = ("Shedrack", "Abel", 23, true);
    println!(
        "mr {} {} is {} years old, his marital status is: {}",
        person.0, person.1, person.2, person.3,
    );
}

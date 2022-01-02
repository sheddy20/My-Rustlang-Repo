pub fn run() {
    let mut message = String::from("Hello ");
    println!("Length: {}", message.len());

    message.push('W');
    message.push_str("orld");
    println!("{}", message);

    for word in message.split_whitespace() {
        println!("{}", word);
    }

    let odd_numbers: [i32; 5] = [1, 3, 5, 7, 9];
    println!("Odd Numbers: {:?}", odd_numbers);

    let person: (&str, &str, i8, bool, f32) = ("Ogden", "Morrow", 58, true, 6.5);
    println!(
        "{} {} is the founder of Oasis, he  is {} years old, marital status: {}, height: {}",
        person.0, person.1, person.2, person.3, person.4,
    );

    println!(
        "{name} likes to play {sport}, he is {age} years old",
        name = "Shedrack",
        sport = "Football",
        age = 23,
    );
}

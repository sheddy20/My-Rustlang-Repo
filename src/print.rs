pub fn run(){

    //Basic formatting
    println!("{} is from {}", "shedrack", "krypton");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "shedrack", "earth", "code");

    //Named Arguments
    println!("{name} likes to play {sport}: ", name = "Shedrack", sport = "Basketball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Debug Traits
    println!("{:?}", (12, "Joe", 45.0, true, 'A'));

    //Basic Maths
    println!("20 + 45: {}", 10 + 45);
}
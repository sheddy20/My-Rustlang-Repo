pub fn run_func() {
    greetings("Hello", "Mason Lockwood", 10);

    let result = add_numbers(10, 15);
    println!("Result: {}", result);
    let num3: i32 = 40;
    let numbers = |num1: i32, num2: i32| num1 * num2 + num3;
    println!("Closure Numbers: {}", numbers(10, 45));
}

fn greetings(greet: &str, name: &str, time: i32) {
    println!(
        "{} {}, nice to meet you, you greeted around {} PM",
        greet, name, time
    );
}

fn add_numbers(num_1: i32, num_2: i32) -> i32 {
    num_1 + num_2
}

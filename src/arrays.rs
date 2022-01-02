use std::mem;

pub fn run() {
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in numbers {
        println!("All numbers: {}", number);
    }

    println!("{}", numbers.len());

    let mut todos: [&str; 5] = ["wash cloth", "gym", "buy bread", "walk", "sing"];
    for tasks in todos {
        println!("Tasks: {}", tasks);
    }
    todos[3] = "Apply for iran visa";
    println!("Task -> 0: {}", todos[0]);

    let details: (&str, &str, &str, i8, bool) = ("Shedrack", "Abel", "Nigeria", 23, false);
    println!(
        "Details Info: {} {} {} {} {}",
        details.0, details.1, details.2, details.3, details.4
    );

    let mut counters: [i32; 6] = [20, 30, 40, 56, 80, 5000];
    counters[0] = 100;
    counters[1] = 300;
    counters[2] = 400;
    counters[3] = 700;
    counters[4] = 900;
    counters[5] = 4000;
    println!("Counters: {:?}", counters);
    println!("Counters Length: {}", counters.len());
    println!("Counters Occupies: {} bytes", mem::size_of_val(&counters));

    let slices: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slices);

    let second_slice: &[i32] = &counters[1..2];
    println!("Counters Slice: {:?}", second_slice);
}

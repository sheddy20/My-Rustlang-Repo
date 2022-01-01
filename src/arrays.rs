pub fn run() {
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for number in numbers {
        println!("All numbers: {}", number);
    }

    let todos: [&str; 5] = ["wash cloth", "gym", "buy bread", "walk", "sing"];
    for tasks in todos {
        println!("Tasks: {}", tasks);
    }
}

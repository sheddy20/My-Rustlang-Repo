pub fn run() {
    let mut vectors_numbers: Vec<i32> = vec![];
    vectors_numbers.push(10);
    vectors_numbers.push(20);
    vectors_numbers.push(30);
    vectors_numbers.push(45);
    println!("Vec Arrays: {:?}", vectors_numbers);
    println!("Vec Length: {}", vectors_numbers.len());
    println!("Vec in bytes {}", std::mem::size_of_val(&vectors_numbers));

    for numbers in vectors_numbers.iter() {
        println!("Numbers: {}", numbers);
    }

    for number in vectors_numbers.iter_mut() {
        *number *= 10;
        println!("Numbers Vectors: {}", number);
    }

    let mut my_tasks: Vec<&str> = vec![];
    my_tasks.push("Wash the clothes");
    my_tasks.push("Fly the plane");
    my_tasks.push("Join the forum");
    my_tasks.push("Register for rustcon");

    for task in my_tasks {
        println!("My Todos: {}", task);
        println!("My Todo In Bytes: {}", std::mem::size_of_val(&task));
    }

    let mut odd_numbers: Vec<i32> = vec![3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23];
    for odd in odd_numbers.iter_mut() {
        *odd *= 2;
        println!("Even Numbers: {}", odd);
    }
    println!("Number: {:?}", odd_numbers);
}

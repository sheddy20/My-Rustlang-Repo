pub fn pointers_ref() {
    let first_numbers = [1, 2, 3];
    let second_numbers = first_numbers;
    println!("Answer: {:?}", (first_numbers, second_numbers));

    let num1: Vec<i32> = vec![10, 20, 30];
    let num2 = &num1;
    println!("Answer: {:?}", (&num1, num2));

    let mut hero = String::with_capacity(5);
    hero.push('B');
    hero.push('r');
    hero.push('u');
    hero.push('c');
    hero.push('e');

    println!("FullName: {}", hero);

    assert_eq!(3, hero.len())
}

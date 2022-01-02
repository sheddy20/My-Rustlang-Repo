pub fn test_run() {
    let numbers: [i32; 4] = [4, 5, 6, 8];
    println!("Numbers: {:?}", numbers);

    let new_nums: &[i32] = &numbers[0..1];
    println!("New Nums: {:?}", new_nums);

    let mut odd_nums: Vec<i32> = vec![1, 3, 5, 7, 9, 11, 13, 15, 21, 23];

    for even_nums in odd_nums.iter_mut() {
        *even_nums *= 2;
        println!("Even Numbers: {}", even_nums);
    }

    println!("Numbers: {:?}", odd_nums);

    let mut counters = 0;

    // loop {
    //     counters += 1;
    //     if counters == 10 {
    //         break;
    //     }
    //     println!("Counters: {}", counters);
    // }

    while counters <= 100 {
        if counters % 15 == 0 {
            println!("FizzBuzz");
        } else if counters % 3 == 0 {
            println!("Fizz");
        } else if counters % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", counters);
        }

        counters += 1;
    }

    for xender in 0..100 {
        if xender % 15 == 0 {
            println!("FizBuzz");
        } else if xender % 3 == 0 {
            println!("Fizz");
        } else if xender % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", counters);
        }
    }
}

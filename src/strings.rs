pub fn run() {
    let mut message = String::from("Hello ");
    // println!("{}", message);

    //get length
    println!("Length: {}", message.len());
    message.push('W');
    message.push_str("orld");
    println!("{}", message);
    println!("Capacity In Bytes: {}", message.capacity());
    println!("Is Empty: {}", message.is_empty());

    let mut flash = String::from("Good Evening ");
    println!("Barry Allen Length: {}", flash.len());

    flash.push('B');
    flash.push_str("arry ");
    flash.push('A');
    flash.push_str("llen ");
    println!("Gideon AI Message: {}", flash);

    //Capacity in bytes
    println!("Capacity In Bytes: {}", flash.capacity());
    //Is Empty Methods
    println!("Is Empty: {}", flash.is_empty());
    //Contains Substrings
    println!("Contain the world 'A': {}", flash.contains("A"));
    //Replace Methods
    println!(
        "Replace the word 'llen' with 'lan': {}",
        flash.replace("llen", "lan")
    );
    //Loop through string by whitespace
    for word in flash.split_whitespace() {
        println!("Whitespace: {}", word);
    }

    //Create String With Capacity
    let mut s = String::with_capacity(10);
    s.push('b');
    s.push('a');
    s.push('r');
    s.push('r');
    s.push('y');

    let mut n = String::with_capacity(5);

    n.push('a');
    n.push('l');
    n.push('l');
    n.push('e');
    n.push('n');

    println!("Super Hero Name: {} {}", s, n);

    //Assertion
    assert_eq!(5, s.len());
    assert_eq!(5, n.len());

    assert_eq!(10, s.capacity());
    assert_eq!(5, n.capacity());
}

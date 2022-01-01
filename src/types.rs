pub fn run() {
    let x = 1;
    println!("{}", x);

    let y = 2.5;
    println!("{}", y);

    let large: i64 = 4545454545454546;
    println!("{}", large);
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);

    let is_married = true;
    let is_greater = 20 > 10;
    let emojis = 'A';
    let face = '\u{1F600}';
    let new_face = '😎';
    println!(
        "Length: {:?}",
        (x, y, is_married, is_greater, emojis, face, new_face)
    );
}

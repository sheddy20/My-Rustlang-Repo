//Traditional Struct
struct Colors {
    red: u8,
    green: u8,
    blue: u8,
}

//Tuple Structs
struct Color(u8, u8, u8);

pub fn struct_run() {
    let mut colors = Colors {
        red: 255,
        green: 0,
        blue: 0,
    };

    colors.red = 200;
    colors.green = 100;
    colors.blue = 50;

    println!("Colors: {} {} {}", colors.red, colors.green, colors.blue);

    let mut color = Color(255, 0, 0);
    color.0 = 30;
    color.1 = 20;
    color.2 = 200;
    println!("Color: {} {} {}", color.0, color.1, color.2);
}

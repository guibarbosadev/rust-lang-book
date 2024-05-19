use std::io;

fn main() {
    let mut width = String::from("");
    let mut height = String::from("");
    let parsed_width: i32;
    let parsed_height: i32;
    let area: i32;

    loop {
        println!("Insert rectangle x: ");
        width.clear();
        io::stdin()
            .read_line(&mut width)
            .expect("Failed getting x value");
        parsed_width = match width.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };

        break;
    }

    loop {
        println!("Insert rectangle y: ");
        height.clear();
        io::stdin()
            .read_line(&mut height)
            .expect("Failed getting y value");
        parsed_height = match height.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };
        break;
    }

    area = calculate_area(parsed_width, parsed_height);

    println!("Area is: {}", &area);
}

fn calculate_area(width: i32, height: i32) -> i32 {
    return width * height;
}

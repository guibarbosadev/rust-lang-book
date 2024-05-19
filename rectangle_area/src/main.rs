use std::io;

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        return &self.width * self.height;
    }
}

fn main() {
    let mut width = String::from("");
    let mut height = String::from("");
    let parsed_width: i32;
    let parsed_height: i32;
    let rectangle: Rectangle;

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

    rectangle = Rectangle {
        width: parsed_width,
        height: parsed_height,
    };

    println!("Area is: {}", rectangle.area());
}

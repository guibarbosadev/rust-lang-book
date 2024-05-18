use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let parsed_x: u32;
    let parsed_y: u32;
    let total: u32;

    loop {
        println!("Insert x: ");
        x.clear();
        io::stdin().read_line(&mut x).expect("Failed reading x");
        parsed_x = match x.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };
        break;
    }

    loop {
        println!("Insert y: ");
        y.clear();
        io::stdin().read_line(&mut y).expect("Failed reading y");
        parsed_y = match y.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };
        break;
    }

    total = sum(parsed_x, parsed_y);

    println!("The sum is: {}", total);
}

fn sum(x: u32, y: u32) -> u32 {
    return x + y;
}

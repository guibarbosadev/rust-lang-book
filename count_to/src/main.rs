use std::io;

fn main() {
    let mut max = String::new();

    loop {
        println!("Insert value to be counted to: ");
        max.clear();
        io::stdin()
            .read_line(&mut max)
            .expect("Failed to read value");
        let max: i32 = match max.trim().parse() {
            Err(_) => continue,
            Ok(num) => num,
        };

        count_to(max);
        break;
    }
}

fn count_to(max: i32) {
    for count in 1..=max {
        println!("{}", count);
    }
}

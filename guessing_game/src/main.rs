use colored::{self, Colorize};
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut guess = String::new();
    let random_number: u32 = rand::thread_rng().gen_range(1..100);
    println!("Correct number: {}", random_number);

    loop {
        println!("Please input your guess: ");
        guess.clear(); // Clear the content of the string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("{}", "Too low".red()),
            Ordering::Greater => println!("{}", "Too high".red()),
            Ordering::Equal => {
                println!("{}", "Correct guess!".green());
                break;
            }
        };
    }
}

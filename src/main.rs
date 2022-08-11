use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Guessing Game
    println!("Guessing Game in Rust!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Enter your number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You guessed right!\n".green());
                break;
            }
            Ordering::Greater => {
                println!("{}", "Too big!\n".red())
            }
            Ordering::Less => {
                println!("{}", "Too less :(\n".red())
            }
        }
    }
}

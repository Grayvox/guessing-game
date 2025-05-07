use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_o) => {}
        Err(_e) => {
            println!("Failed to read line.")
        }
    }

    println!("You guessed: {guess}");
}

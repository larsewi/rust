use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess = guess.trim();
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Hold your horses; '{guess}' is not a number.");
                continue;
            },
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

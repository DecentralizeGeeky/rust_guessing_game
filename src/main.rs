use std::cmp::Ordering;
use std::io;

use rand::RngExt;

fn main() {
    println!("Guess the number!");

    // rand 0.10 uses the new rng() entry point for the default thread-local generator.
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);
    let mut guess_count = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read a full line from stdin so the player can submit a guess interactively.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Ignore invalid input and keep the game running.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Count only valid guesses so the final result reflects actual attempts.
        guess_count += 1;

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! You guessed it in {guess_count} guesses.");
                break;
            }
        }
    }
}
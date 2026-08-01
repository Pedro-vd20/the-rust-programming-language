use std::cmp::Ordering;
use std::io;

use rand::Rng;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    println!("Guess the number!");
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        if guess.trim() == "quit" || guess.trim() == "exit" {
            println!("Exiting the game. Goodbye!");
            break;
        }

        // Shadows the str guess - POSSIBLE BAD PRACTICE
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number: \"{}\"", guess.trim());
                println!("To exit the game, type \"quit\" or \"exit\".");
                continue;
            },
        };


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

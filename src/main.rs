use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number (1-100)!");

    let mut guesses = 0;

    loop {
        println!("Enter your guess:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        guesses += 1;

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Correct! You got it in {guesses} guesses !");
                break;
            }
        }
    }
}

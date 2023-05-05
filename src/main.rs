use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("Guess The Number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Please Enter Your Guess...");

        let mut guess = String::new();

        // Accept input from user and bind with the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Convert the guess number to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(guessed_number) => guessed_number,
            Err(_) => continue,
        };

        println!("You Guessed the Number: {guess}");

        // Comart The guessed number with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small Number!"),
            Ordering::Equal => {
                println!("Congrats, You Guessed The Right Number");
                break;
            },
            Ordering::Greater => println!("Too big!")
        }
    }
}

use std::io;

fn main() {
    println!("Guess The Number!");
    println!("Please Enter Your Guess...");

    let mut guess = String::new();

    // Accept input from user and bind with the guess variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You Guessed the Number: {guess}");

}

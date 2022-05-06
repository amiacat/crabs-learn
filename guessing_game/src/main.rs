use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..11);

    println!("The secret number is {secret_number}");

    println!("Put your guessing: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .unwrap();

    println!("You guessed {guess}");
}

use rand::Rng;
use std::cmp::Ordering; // Ordering is an enum type
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // match is powerful expression for declaring comparasion series
    // comparing is not working because guess is type string and secret_number is the number type
    // rust is  strong static type system
    match guess.cmp(&secret_number) { // compare guess and secret_number
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

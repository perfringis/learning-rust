use std::io; // import of input/output libary from standard library

fn main() { // the main fuction
    println!("Guess the number!"); // println! is a macro! It's not a function!

    println!("Please input your guess.");

    // declaration of mutable variable
    // String::new() <- returns new instance of String type
    // ::new <- means invking method from String type
    let mut guess = String::new();

    // declaration of immutable variable
    let apples = 5;
    println!("Number of apples: {apples}");

    io::stdin() // returns input handle
        .read_line(&mut guess) // take standard input and save to variable // "&mut guess" is reference is not creating a copy!
        .expect("Failed to read line"); // when read_line is returning error

    println!("You guessed: {guess}"); // format string when passing variable
}

// 1. Variables.
// Variables are inmmutable by deafult!
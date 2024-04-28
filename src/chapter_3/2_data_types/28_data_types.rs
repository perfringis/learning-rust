// Control Flow
// Repetition with Loops

// Returning Values from Loops

fn main() {
    let mut counter = 0; // declaration of mutable variable

    let result = loop { // assigning returned value for loop to result variable
        counter += 1;

        if counter == 10 {
            break counter * 2; // break keyword works like return keyword and returns counter * 2
        }
    };

    println!("The result is {result}");
}
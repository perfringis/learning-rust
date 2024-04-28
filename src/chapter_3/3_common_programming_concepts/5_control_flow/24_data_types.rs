// Control Flow
// Handling Multiple Conditions with else if

fn main() {
    let number = 6;

    // chaining several if statements
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 { // else if like in c like languages
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else { // else keyword like in c like languages
        println!("number is not divisible by 4, 3, or 2");
    }
}
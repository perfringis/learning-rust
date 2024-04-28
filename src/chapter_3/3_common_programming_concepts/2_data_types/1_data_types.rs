fn main() {
    // compiler is not able to recognize variable type
    let guess = "42".parse().expect("Not a number!");

    // working code
    let guess_2: u32 = "42".parse().expect("Not a number!");
}

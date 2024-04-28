// Functions with Return Values

// Rust don't name returned value, but we have to specify returned type
fn five() -> i32 {
    5 // it is expression. This like doesn't have semicolon sign
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

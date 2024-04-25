fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // assigning second time to the imutable variable is impossible
    println!("The value of x is: {x}");
}

// continue: https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#variables-and-mutability